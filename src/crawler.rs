use super::entry;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Worker(#[from] worker::Error),
    #[error("Failed to strip base prefix `{0}` from `{1}`")]
    StripPrefix(std::path::PathBuf, std::path::PathBuf),
    #[error("Full collision detected for `{0}`")]
    FullCollision(std::path::PathBuf),
}

pub fn crawl(path: &std::path::Path, pool: &rayon::ThreadPool) -> Result<Vec<entry::Entry>, Error> {
    log::info!("Indexing {}", path.display());
    let start = std::time::Instant::now();

    let entries = pool.install(|| {
        let (sender, receiver) = std::sync::mpsc::channel();

        let path_clone = path.to_path_buf();
        worker::dispatcher::dispatch(path_clone, sender);

        accumulate(&receiver, path)
    })?;

    log::info!(
        "Finished indexing {} items for {} in {:?}",
        entries.len(),
        path.display(),
        start.elapsed(),
    );

    Ok(entries)
}

fn accumulate(
    receiver: &std::sync::mpsc::Receiver<worker::Message>,
    base: &std::path::Path,
) -> Result<Vec<entry::Entry>, Error> {
    let mut paths = Vec::new();
    let start = std::time::Instant::now();

    while let Ok(result) = receiver.recv() {
        let (hash, path) = result?;

        let entry = entry::Entry::new(&path, base, hash)
            .map_err(|_| Error::StripPrefix(base.to_path_buf(), path))?;

        let Err(index) = paths.binary_search(&entry) else {
            return Err(Error::FullCollision(entry.path));
        };

        paths.insert(index, entry);

        let len = paths.len();
        if len & (2048 - 1) == 0 {
            let elapsed = start.elapsed().as_secs();
            if elapsed > 0 {
                log::debug!("Indexed {len} items at {} items/s", len as u64 / elapsed);
            }
        }
    }

    Ok(paths)
}

mod worker {
    use crate::entry::Hash;
    pub type Message = Result<(Hash, std::path::PathBuf), Error>;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Dispatcher error: {0}")]
        Dispatcher(#[from] dispatcher::Error),
        #[error("Hasher error: {0}")]
        Hasher(#[from] hasher::Error),
    }

    pub mod dispatcher {
        use super::{Error as WorkerError, Message};

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error("Could not read directory `{0}`: {1}")]
            DirUnreadable(std::path::PathBuf, std::io::Error),
            #[error("Could not read directory entry for `{0}`: {1}")]
            EntryUnreadable(std::path::PathBuf, std::io::Error),
        }

        pub fn dispatch(path: std::path::PathBuf, sender: std::sync::mpsc::Sender<Message>) {
            rayon::spawn(|| {
                if let Err(e) = dispatch_internal(path, sender) {
                    log::warn!("Failed to send error from dispatcher: {e}");
                }
            });
        }

        fn dispatch_internal(
            path: std::path::PathBuf,
            sender: std::sync::mpsc::Sender<Message>,
        ) -> Result<(), Error> {
            let dir = match scan_dir(&path) {
                Ok(dir) => dir,
                Err(e) => {
                    // Always quit on error.
                    // If the send succeeds, return Ok(), levaing the accumulator to abort the
                    // execution.
                    // If the send fails, repeat the error back out.
                    return sender
                        .send(Err(WorkerError::Dispatcher(e)))
                        .map_err(|e| match e.0 {
                            Err(WorkerError::Dispatcher(e)) => e,
                            Ok(_) | Err(WorkerError::Hasher(_)) => {
                                unreachable!("Cannot fail to send a dispatcher error")
                            }
                        });
                }
            };

            for path in dir {
                let is_dir = if path.is_symlink() {
                    if let Ok(meta) = path.metadata() {
                        meta.is_dir()
                    } else {
                        log::warn!("Found broken symlink at {}", path.display());
                        continue;
                    }
                } else {
                    path.is_dir()
                };

                let sender = sender.clone();
                if is_dir {
                    dispatch_internal(path, sender)?;
                } else {
                    rayon::spawn(move || super::hasher::hash(path, sender));
                }
            }

            drop(path);
            drop(sender);
            Ok(())
        }

        fn scan_dir(path: &std::path::Path) -> Result<Vec<std::path::PathBuf>, Error> {
            path.read_dir()
                .map_err(|e| Error::DirUnreadable(path.to_path_buf(), e))?
                .map(|entry| entry.map(|p| p.path()))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| Error::EntryUnreadable(path.to_path_buf(), e))
        }
    }

    mod hasher {
        use super::{Error as WorkerError, Message};

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error("Could not open file {0}: {1}")]
            CannotOpen(std::path::PathBuf, std::io::Error),
            #[error("Could not read file {0}: {1}")]
            CannotRead(std::path::PathBuf, std::io::Error),
            #[error("Could not send entry: {0}")]
            Send(std::path::PathBuf),
        }

        pub fn hash(path: std::path::PathBuf, sender: std::sync::mpsc::Sender<Message>) {
            if let Err(e) = hash_internal(path, sender) {
                match e {
                    Error::Send(path) => {
                        log::warn!("Failed to send entry from hasher: {}", path.display());
                    }
                    e => log::warn!("Failed to send error from hasher: {e}"),
                }
            }
        }

        fn hash_internal(
            path: std::path::PathBuf,
            sender: std::sync::mpsc::Sender<Message>,
        ) -> Result<(), Error> {
            macro_rules! send {
                ($value: expr) => {
                    sender.send($value).map_err(|e| match e.0 {
                        Ok((_, path)) => Error::Send(path),
                        Err(WorkerError::Hasher(e)) => e,
                        Err(WorkerError::Dispatcher(_)) => {
                            unreachable!("Cannot fail to send a dispatcher error")
                        }
                    })
                };
            }

            macro_rules! unwrap {
                ($match: expr) => {
                    match $match {
                        Ok(ok) => ok,
                        Err(e) => return send!(Err(e.into())),
                    }
                };
            }

            use md5::Digest;

            let mut file = unwrap!(std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(&path)
                .map_err(|e| Error::CannotOpen(path.clone(), e)));

            let mut hasher = md5::Md5::new();
            let mut buffer = [0; 1024 * 4];

            loop {
                use std::io::Read;

                let bytes = unwrap!(file
                    .read(&mut buffer)
                    .map_err(|e| Error::CannotRead(path.clone(), e)));

                if bytes == 0 {
                    break;
                }

                hasher.update(&buffer[..bytes]);
            }

            send!(Ok((super::Hash::new(hasher.finalize()), path)))?;

            drop(sender);
            Ok(())
        }
    }
}
