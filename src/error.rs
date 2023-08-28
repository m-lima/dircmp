pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Thread(#[from] crate::thread::Error),
    #[error(transparent)]
    Hasher(#[from] crate::hasher::Error),
}
