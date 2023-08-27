use crate::{Error, Fatal, Result};

type Entry = ([u8; 16], std::path::PathBuf);

pub struct DirIndex {
    children: Vec<Entry>,
}

impl DirIndex {
    pub fn new(path: &std::path::Path, pool: &threadpool::ThreadPool) -> Result<Self> {
        let accumulator = init(path, pool)?;

        let children = accumulator
            .join()
            .map_err(Fatal::JoinError)
            .map_err(Error::from)??;

        Ok(Self { children })
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }
}

fn scan_dir(path: &std::path::Path) -> Result<Vec<std::path::PathBuf>> {
    path.read_dir()
        .map_err(|e| Error::PathUnreadable(path.to_path_buf(), e))?
        .map(|entry| entry.map(|p| p.path()))
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|e| Error::PathUnreadable(path.to_path_buf(), e))
}

fn init(
    path: &std::path::Path,
    pool: &threadpool::ThreadPool,
) -> Result<std::thread::JoinHandle<Result<Vec<Entry>>>> {
    let (sender, receiver) = std::sync::mpsc::channel();

    fallible_crawl(path, sender.clone(), pool.clone())?;
    let accumulator = std::thread::spawn(move || accumulator(receiver));

    Ok(accumulator)
}

fn accumulator(receiver: std::sync::mpsc::Receiver<Result<Entry>>) -> Result<Vec<Entry>> {
    let mut paths = Vec::new();
    while let Ok(result) = receiver.recv() {
        let entry = match result {
            Ok(entry) => entry,
            Err(e) => return Err(e),
        };

        let Err(index) = paths.binary_search(&entry) else {
            return Err(Error::Fatal(Fatal::FullCollision(entry.1)));
        };

        paths.insert(index, entry);
        if paths.len() & ((1024 * 8) - 1) == 0 {
            eprintln!("Indexed {} items", paths.len());
        }
    }

    drop(receiver);
    Ok(paths)
}

fn fallible_crawl(
    path: &std::path::Path,
    sender: std::sync::mpsc::Sender<Result<Entry>>,
    pool: threadpool::ThreadPool,
) -> std::result::Result<(), Fatal> {
    let dir = match scan_dir(path) {
        Ok(dir) => dir,
        Err(e) => {
            let err_string = e.to_string();
            sender
                .send(Err(e))
                .map_err(|_| Fatal::SendError(err_string))?;
            return Ok(());
        }
    };

    for path in dir {
        if path.is_dir() {
            let sender = sender.clone();
            let pool_handle = pool.clone();
            pool.execute(move || crawl(&path, sender, pool_handle));
        } else {
            use md5::Digest;

            let mut file = match std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(&path)
                .map_err(|e| Error::CannotRead(path.clone(), e))
            {
                Ok(file) => file,
                Err(e) => {
                    let err_string = e.to_string();
                    sender
                        .send(Err(e))
                        .map_err(|_| Fatal::SendError(err_string))?;
                    return Ok(());
                }
            };

            let mut hasher = md5::Md5::new();
            let mut buffer = [0; 1024 * 4];

            loop {
                use std::io::Read;

                let bytes = match file
                    .read(&mut buffer)
                    .map_err(|e| Error::CannotRead(path.clone(), e))
                {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        let err_string = e.to_string();
                        sender
                            .send(Err(e))
                            .map_err(|_| Fatal::SendError(err_string))?;
                        return Ok(());
                    }
                };

                if bytes == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes]);
            }

            sender
                .send(Ok((hasher.finalize().into(), path)))
                .map_err(|_| Fatal::SendHash)?;
        }
    }

    drop(sender);
    drop(pool);
    Ok(())
}

fn crawl(
    path: &std::path::Path,
    sender: std::sync::mpsc::Sender<Result<Entry>>,
    pool: threadpool::ThreadPool,
) {
    if let Err(e) = fallible_crawl(path, sender, pool) {
        eprintln!("Executor error: {e}");
    }
}
