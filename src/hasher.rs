type Entry = ([u8; 16], std::path::PathBuf);

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
    path: std::path::PathBuf,
    children: Vec<Entry>,
}

impl Index {
    pub fn new(path: std::path::PathBuf, pool: &rayon::ThreadPool) -> Result<Self, Error> {
        log::info!("Indexing {}", path.display());
        let children = pool.install(|| init(&path))?;
        log::info!(
            "Finished indexing {} items for {}",
            children.len(),
            path.display()
        );
        Ok(Self { path, children })
    }

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }
}

fn init(path: &std::path::Path) -> Result<Vec<Entry>, Error> {
    let (sender, receiver) = std::sync::mpsc::channel();
    crawler::crawl(path, sender)?;
    accumulate(&receiver, path)
}

fn accumulate(
    receiver: &std::sync::mpsc::Receiver<Result<Entry, crawler::Error>>,
    base: &std::path::Path,
) -> Result<Vec<Entry>, Error> {
    let mut paths = Vec::new();
    while let Ok(result) = receiver.recv() {
        let entry = match result {
            Ok((hash, path)) => (
                hash,
                match path.strip_prefix(base) {
                    Ok(path) => path.to_path_buf(),
                    Err(_) => return Err(Error::StripPrefix(base.to_path_buf(), path)),
                },
            ),
            Err(e) => return Err(Error::Crawler(e)),
        };

        let Err(index) = paths.binary_search(&entry) else {
            return Err(Error::FullCollision(entry.1));
        };

        paths.insert(index, entry);
        if paths.len() & (1024 - 1) == 0 {
            log::debug!("Indexed {} items", paths.len());
        }
    }

    Ok(paths)
}

mod crawler {
    use super::Entry;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Could not read file {0}: {1}")]
        CannotRead(std::path::PathBuf, std::io::Error),
        #[error("Could not read directory `{0}`: {1}")]
        DirUnreadable(std::path::PathBuf, std::io::Error),
        #[error("Could not read directory entry for `{0}`: {1}")]
        EntryUnreadable(std::path::PathBuf, std::io::Error),
        #[error("Entry: {0}")]
        Send(std::path::PathBuf),
    }

    pub fn crawl(
        path: &std::path::Path,
        sender: std::sync::mpsc::Sender<Result<Entry, Error>>,
    ) -> std::result::Result<(), Error> {
        macro_rules! send {
            ($value: expr) => {
                sender.send($value).map_err(|e| match e.0 {
                    Ok((_, path)) => Error::Send(path),
                    Err(e) => e,
                })
            };
        }
        macro_rules! unwrap {
            ($match: expr) => {
                match $match {
                    Ok(ok) => ok,
                    Err(e) => return send!(Err(e)),
                }
            };
        }

        for path in unwrap!(scan_dir(path)) {
            if path.is_dir() {
                let sender = sender.clone();
                rayon::spawn(move || crawl_internal(&path, sender));
            } else {
                use md5::Digest;

                let mut file = unwrap!(std::fs::OpenOptions::new()
                    .read(true)
                    .write(false)
                    .create(false)
                    .open(&path)
                    .map_err(|e| Error::CannotRead(path.clone(), e)));

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

                send!(Ok((hasher.finalize().into(), path)))?;
            }
        }

        drop(sender);
        Ok(())
    }

    pub fn crawl_internal(
        path: &std::path::Path,
        sender: std::sync::mpsc::Sender<Result<Entry, Error>>,
    ) {
        if let Err(e) = crawl(path, sender) {
            match e {
                Error::Send(path) => {
                    log::error!("Failed to send entry from crawler: {}", path.display());
                }
                e => log::error!("Failed to send error from crawler: {e}"),
            }
        }
    }

    fn scan_dir(path: &std::path::Path) -> Result<Vec<std::path::PathBuf>, Error> {
        path.read_dir()
            .map_err(|e| Error::DirUnreadable(path.to_path_buf(), e))?
            .map(|entry| entry.map(|p| p.path()))
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::EntryUnreadable(path.to_path_buf(), e))
    }
}
