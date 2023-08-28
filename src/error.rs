pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Args(#[from] crate::args::Error),
    // #[error(transparent)]
    // Thread(#[from] crate::thread::Error),
    #[error(transparent)]
    Hasher(#[from] crate::hasher::Error),
    // #[error(transparent)]
    // Path(#[from] Path),
    // #[error("Fatal error: {0}")]
    // Internal(#[from] Internal),
}

// #[derive(Debug, thiserror::Error)]
// pub enum Path {
//     #[error("Path is not a file: {0}")]
//     PathNotFile(std::path::PathBuf),
//     #[error("Path does not contain a base name: {0}")]
//     PathWithoutBasename(std::path::PathBuf),
//     #[error("File does not contain a parent: {0}")]
//     FileWithoutParent(std::path::PathBuf),
//     #[error("Failed to write to stdout: {0}")]
//     Io(#[from] std::io::Error),
//     #[error("Inconsistent mutal directories found")]
//     InconsistentSize,
//     #[error("Could not read file {0}: {1}")]
//     CannotRead(std::path::PathBuf, std::io::Error),
//     #[error("Both path do not exist: `{0}` `{1}`")]
//     TwoMissingFiles(std::path::PathBuf, std::path::PathBuf),
// }
//
// #[derive(Debug, thiserror::Error)]
// pub enum Internal {
//     #[error("Failed to send hash from crawler thread")]
//     SendHash,
//     #[error("Failed to send error from crawler thread: {0}")]
//     SendError(String),
//     #[error("Error in spawned thread")]
//     JoinError(Box<dyn std::any::Any + Send + 'static>),
//     #[error("Full collision detected for `{0}`")]
//     FullCollision(std::path::PathBuf),
// }
