mod entry;
mod error;
mod index;
mod thread;

pub use error::Error;
pub use index::Index;

/// Compares two directories [`left`](std::path::PathBuf) and [`right`](std::path::PathBuf)
/// returning the [`Index`](index::Index)
///
/// # Errors
///
/// This is a fallible process and will fail-fast.
/// In rare occasions, i.e. when worker threads are not able to send errors back up orchestrator,
/// errors will be globbed and simply written into an [`Error`](log::Level::Error) log entry.
pub fn compare(
    left: std::path::PathBuf,
    right: std::path::PathBuf,
) -> Result<(index::Index, index::Index), error::Error> {
    let pool = thread::pool()?;
    let mut left = index::Index::new(left, &pool)?;
    let mut right = index::Index::new(right, &pool)?;
    left.compare(&right, &pool);
    right.compare(&left, &pool);

    Ok((left, right))
}
