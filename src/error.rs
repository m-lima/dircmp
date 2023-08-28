#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Thread(#[from] crate::thread::Error),
    #[error(transparent)]
    Index(#[from] crate::index::Error),
}
