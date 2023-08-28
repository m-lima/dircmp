use super::entry;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Crawler(#[from] crawler::Error),
    #[error("Failed to strip base prefix `{0}` from `{1}`")]
    StripPrefix(std::path::PathBuf, std::path::PathBuf),
    #[error("Full collision detected for `{0}`")]
    FullCollision(std::path::PathBuf),
}

pub struct Index {
    pub(crate) path: std::path::PathBuf,
    pub(crate) children: Vec<entry::Entry>,
}

impl Index {
    #[must_use]
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    #[must_use]
    pub fn children(&self) -> &[entry::Entry] {
        &self.children
    }

    #[must_use]
    pub fn decompose(self) -> (std::path::PathBuf, Vec<entry::Entry>) {
        (self.path, self.children)
    }
}

impl Index {
    pub(crate) fn new(path: std::path::PathBuf, pool: &rayon::ThreadPool) -> Result<Self, Error> {
        log::info!("Indexing {}", path.display());
        let start = std::time::Instant::now();
        let children = pool.install(|| init(&path))?;
        log::info!(
            "Finished indexing {} items for {} in {:?}",
            children.len(),
            path.display(),
            start.elapsed(),
        );

        Ok(Self { path, children })
    }
}

fn init(path: &std::path::Path) -> Result<Vec<entry::Entry>, Error> {
    let (sender, receiver) = std::sync::mpsc::channel();
    let crawl_path = path.to_path_buf();
    rayon::spawn(|| crawler::crawl(crawl_path, sender));
    accumulate(&receiver, path)
}

fn accumulate(
    receiver: &std::sync::mpsc::Receiver<Result<crawler::Entry, crawler::Error>>,
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

mod crawler {
    macro_rules! send {
        ($sender: ident, $value: expr) => {
            $sender.send($value).map_err(|e| match e.0 {
                Ok((_, path)) => Error::Send(path),
                Err(e) => e,
            })
        };
    }

    macro_rules! unwrap {
        ($sender: ident, $match: expr) => {
            match $match {
                Ok(ok) => ok,
                Err(e) => return send!($sender, Err(e)),
            }
        };
    }

    pub type Entry = (super::entry::Hash, std::path::PathBuf);

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Hasher could not open file {0}: {1}")]
        CannotOpen(std::path::PathBuf, std::io::Error),
        #[error("Hasher could not read file {0}: {1}")]
        CannotRead(std::path::PathBuf, std::io::Error),
        #[error("Crawler could not read directory `{0}`: {1}")]
        DirUnreadable(std::path::PathBuf, std::io::Error),
        #[error("Crawler could not read directory entry for `{0}`: {1}")]
        EntryUnreadable(std::path::PathBuf, std::io::Error),
        #[error("Entry: {0}")]
        Send(std::path::PathBuf),
    }

    pub fn crawl(path: std::path::PathBuf, sender: std::sync::mpsc::Sender<Result<Entry, Error>>) {
        if let Err(e) = crawl_internal(path, sender) {
            log::error!("Failed to send error from crawler: {e}");
        }
    }

    fn crawl_internal(
        path: std::path::PathBuf,
        sender: std::sync::mpsc::Sender<Result<Entry, Error>>,
    ) -> Result<(), Error> {
        for path in unwrap!(sender, scan_dir(&path)) {
            if path.is_symlink() {
                let Ok(metadata) = path.metadata() else {
                        log::warn!("Found broken symlink at {}", path.display());
                        continue;
                };

                let sender = sender.clone();
                if metadata.is_dir() {
                    crawl_internal(path, sender)?;
                } else {
                    rayon::spawn(move || hash(path, sender));
                }
            } else {
                let sender = sender.clone();
                if path.is_dir() {
                    crawl_internal(path, sender)?;
                } else {
                    rayon::spawn(move || hash(path, sender));
                }
            }
        }

        drop(path);
        drop(sender);
        Ok(())
    }

    fn hash(path: std::path::PathBuf, sender: std::sync::mpsc::Sender<Result<Entry, Error>>) {
        if let Err(e) = hash_internal(path, sender) {
            match e {
                Error::Send(path) => {
                    log::error!("Failed to send entry from hasher: {}", path.display());
                }
                e => log::error!("Failed to send error from hasher: {e}"),
            }
        }
    }

    fn hash_internal(
        path: std::path::PathBuf,
        sender: std::sync::mpsc::Sender<Result<Entry, Error>>,
    ) -> Result<(), Error> {
        use md5::Digest;

        let mut file = unwrap!(
            sender,
            std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(&path)
                .map_err(|e| Error::CannotOpen(path.clone(), e))
        );

        let mut hasher = md5::Md5::new();
        let mut buffer = [0; 1024 * 4];

        loop {
            use std::io::Read;

            let bytes = unwrap!(
                sender,
                file.read(&mut buffer)
                    .map_err(|e| Error::CannotRead(path.clone(), e))
            );

            if bytes == 0 {
                break;
            }

            hasher.update(&buffer[..bytes]);
        }

        send!(
            sender,
            Ok((super::entry::Hash::new(hasher.finalize()), path))
        )?;

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
