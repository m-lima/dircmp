mod crawler;
mod entry;
mod linker;
mod thread;

pub use entry::{Directory, Entry, Hash, Status};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Thread(#[from] thread::Error),
    #[error(transparent)]
    Crawler(#[from] crawler::Error),
}

/// Compares two directories [`left`](std::path::PathBuf) and [`right`](std::path::PathBuf)
/// returning the [`Directory`](entry::Directory)
///
/// # Errors
///
/// This is a fallible process and will fail-fast.
/// In rare occasions, i.e. when worker threads are not able to send errors back up to the
/// accumulator, errors will be globbed and simply written into an
/// [`Error`](log::Level::Error) log entry.
pub fn compare(
    left: std::path::PathBuf,
    right: std::path::PathBuf,
) -> Result<(entry::Directory, entry::Directory), Error> {
    let pool = thread::pool()?;
    let mut left_entries = crawler::crawl(&left, &pool)?;
    let mut right_entries = crawler::crawl(&right, &pool)?;

    let empty_hash = entry::Hash::new(md5::Digest::finalize(<md5::Md5 as md5::Digest>::new()));

    linker::first_pass(&mut left_entries, &mut right_entries, &empty_hash, &pool);
    linker::second_pass(&mut left_entries, &mut right_entries, &empty_hash, &pool);

    let left = entry::Directory::new(left, left_entries);
    let right = entry::Directory::new(right, right_entries);

    Ok((left, right))
}
