mod error;
mod index;
mod matches;
mod thread;

/// Compares two directories [`left`](std::path::PathBuf) and [`right`](std::path::PathBuf)
/// returning the [`Matches`](compare::Matches)
///
/// # Errors
///
/// This is a fallible process and will fail-fast.
/// In rare occasions, i.e. when worker threads are not able to send errors back up orchestrator,
/// errors will be globbed and simply written into an [`Error`](log::Level::Error) log entry.
pub fn compare(left: std::path::PathBuf, right: std::path::PathBuf) -> Result<(), error::Error> {
    let pool = thread::pool()?;
    let left = index::Index::new(left, &pool)?;
    let right = index::Index::new(right, &pool)?;
    let _ = matches::Matches::new(left, right, &pool)?;

    Ok(())
}
