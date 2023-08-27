pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Expect two directories to compare. Got {0}")]
    MissingParameter(u8),
    #[error("Path does not exist: {0}")]
    PathDoesNotExist(std::path::PathBuf),
    #[error("Path is not a directory: {0}")]
    PathNotDirectory(std::path::PathBuf),
    #[error("Path is not a file: {0}")]
    PathNotFile(std::path::PathBuf),
    #[error("Path does not contain a base name: {0}")]
    PathWithoutBasename(std::path::PathBuf),
    #[error("Could not read path `{0}`: {1}")]
    PathUnreadable(std::path::PathBuf, std::io::Error),
    #[error("File does not contain a parent: {0}")]
    FileWithoutParent(std::path::PathBuf),
    #[error("Failed to write to stdout: {0}")]
    Io(#[from] std::io::Error),
    #[error("Inconsistent mutal directories found")]
    InconsistentSize,
    #[error("Could not read file {0}: {1}")]
    CannotRead(std::path::PathBuf, std::io::Error),
    #[error("Both path do not exist: `{0}` `{1}`")]
    TwoMissingFiles(std::path::PathBuf, std::path::PathBuf),
    #[error("Fatal error: {0}")]
    Fatal(#[from] Fatal),
}

#[derive(Debug, thiserror::Error)]
pub enum Fatal {
    #[error("Failed to send hash from crawler thread")]
    SendHash,
    #[error("Failed to send error from crawler thread: {0}")]
    SendError(String),
    #[error("Error in spawned thread")]
    JoinError(Box<dyn std::any::Any + Send + 'static>),
    #[error("Full collision detected for `{0}`")]
    FullCollision(std::path::PathBuf),
}
