use super::entry;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Scanner error: {0}")]
    Scanner(#[from] worker::ScannerError),
    #[error("Hasher error: {0}")]
    Hasher(#[from] worker::HasherError),
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
        worker::scanner::scan(path_clone, sender);

        accumulate(&receiver, path)
    })?;

    log::info!(
        "Finished indexing {} items for {} in {:?}",
        entries.len(),
        path.display(),
        start.elapsed(),
    );

    if let Some(first) = entries.first() {
        assert!(
            entries
                .iter()
                .scan(first, |state, curr| {
                    let result = curr >= state;
                    *state = curr;
                    Some(result)
                })
                .all(|a| a),
            "Entries are not sorted"
        );

        log::info!(
            "Finished indexing {} items for {} in {:?}",
            entries.len(),
            path.display(),
            start.elapsed(),
        );
    }

    Ok(entries)
}

fn accumulate(
    receiver: &std::sync::mpsc::Receiver<worker::Message>,
    base: &std::path::Path,
) -> Result<Vec<entry::Entry>, Error> {
    let mut paths = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    let mut total = 0;
    let mut hashes = 0;
    let mut done = false;
    let start = std::time::Instant::now();

    while let Ok(message) = receiver.recv() {
        let (hash, path) = match message {
            worker::Message::Scanner(worker::ScannerMessage::Error(e)) => return Err(e.into()),
            worker::Message::Hasher(worker::HasherMessage::Error(e)) => return Err(e.into()),
            worker::Message::Scanner(worker::ScannerMessage::Queued) => {
                total += 1;
                continue;
            }
            worker::Message::Scanner(worker::ScannerMessage::Done) => {
                done = true;
                continue;
            }
            worker::Message::Hasher(worker::HasherMessage::Hash(hash, path)) => (hash, path),
        };

        let bucket = usize::from(hash.first_byte() >> 3);
        let bucket = unsafe { paths.get_unchecked_mut(bucket) };

        let entry = entry::Entry::new(&path, base, hash)
            .map_err(|_| Error::StripPrefix(base.to_path_buf(), path))?;

        let Err(index) = bucket.binary_search(&entry) else {
            return Err(Error::FullCollision(entry.path));
        };

        bucket.insert(index, entry);
        hashes += 1;

        if hashes & (2048 - 1) == 0 {
            let elapsed = start.elapsed().as_secs();
            if elapsed > 0 {
                log::debug!(
                    "Indexed {hashes}/{total} [{percentage}%] items at {rate} items/s{scanning}",
                    rate = hashes / elapsed,
                    percentage = hashes * 100 / total,
                    scanning = if done { "" } else { " (still scanning)" },
                );
            }
        }
    }

    Ok(paths.into_iter().flatten().collect())
}

mod worker {
    use crate::entry::Hash;
    pub use hasher::{Error as HasherError, Message as HasherMessage};
    pub use scanner::{Error as ScannerError, Message as ScannerMessage};

    pub enum Message {
        Scanner(scanner::Message),
        Hasher(hasher::Message),
    }

    impl From<scanner::Message> for Message {
        fn from(value: scanner::Message) -> Self {
            Self::Scanner(value)
        }
    }

    impl From<hasher::Message> for Message {
        fn from(value: hasher::Message) -> Self {
            Self::Hasher(value)
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum Error {}

    pub mod scanner {
        use super::Message as WorkerMessage;

        pub enum Message {
            Queued,
            Done,
            Error(Error),
        }

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error("Could not read directory `{0}`: {1}")]
            DirUnreadable(std::path::PathBuf, std::io::Error),
            #[error("Could not read directory entry for `{0}`: {1}")]
            EntryUnreadable(std::path::PathBuf, std::io::Error),
            #[error("Failed to send queue signal")]
            Send,
        }

        pub fn scan(path: std::path::PathBuf, sender: std::sync::mpsc::Sender<WorkerMessage>) {
            rayon::spawn(move || {
                if let Err(e) = scan_internal(path, sender.clone()) {
                    log::warn!("Failed to send error from scanner: {e}");
                }
                if let Err(e) = sender.send(Message::Done.into()) {
                    log::warn!("Failed to send done signal from scanner: {e}");
                }
            });
        }

        fn scan_internal(
            path: std::path::PathBuf,
            sender: std::sync::mpsc::Sender<WorkerMessage>,
        ) -> Result<(), Error> {
            let dir = match scan_dir(&path) {
                Ok(dir) => dir,
                Err(e) => {
                    // Always quit on error.
                    // If the send succeeds, return Ok(), levaing the accumulator to abort the
                    // execution.
                    // If the send fails, repeat the error back out.
                    return sender
                        .send(Message::Error(e).into())
                        .map_err(|e| match e.0 {
                            WorkerMessage::Scanner(Message::Error(e)) => e,
                            _ => {
                                unreachable!(
                                    "Cannot fail to send anything other than a scanner::Message::Error"
                                )
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
                    scan_internal(path, sender)?;
                } else {
                    sender.send(Message::Queued.into()).map_err(|e| match e.0 {
                        WorkerMessage::Scanner(Message::Queued) => Error::Send,
                        _ => unreachable!(
                            "Cannot fail to send anything other than a scanner::Message::Queue"
                        ),
                    })?;
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
        use super::{Hash, Message as WorkerMessage};

        pub enum Message {
            Hash(Hash, std::path::PathBuf),
            Error(Error),
        }

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error("Could not open file {0}: {1}")]
            CannotOpen(std::path::PathBuf, std::io::Error),
            #[error("Could not read file {0}: {1}")]
            CannotRead(std::path::PathBuf, std::io::Error),
            #[error("Could not send entry: {0}")]
            Send(std::path::PathBuf),
        }

        pub fn hash(path: std::path::PathBuf, sender: std::sync::mpsc::Sender<WorkerMessage>) {
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
            sender: std::sync::mpsc::Sender<WorkerMessage>,
        ) -> Result<(), Error> {
            macro_rules! send {
                ($value: expr) => {
                    sender.send($value.into()).map_err(|e| match e.0 {
                        WorkerMessage::Hasher(Message::Hash(_, path)) => Error::Send(path),
                        WorkerMessage::Hasher(Message::Error(e)) => e,
                        WorkerMessage::Scanner(_) => {
                            unreachable!(
                                "Cannot fail to send anything other than a hasher::Message"
                            )
                        }
                    })
                };
            }

            macro_rules! unwrap {
                ($match: expr) => {
                    match $match {
                        Ok(ok) => ok,
                        Err(e) => return send!(Message::Error(e.into())),
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

            send!(Message::Hash(super::Hash::new(hasher.finalize()), path))?;

            drop(sender);
            Ok(())
        }
    }
}
