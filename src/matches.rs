use super::index;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("None")]
    None,
}

pub struct Matches;

impl Matches {
    pub fn new(
        left: index::Index,
        right: index::Index,
        _pool: &rayon::ThreadPool,
    ) -> Result<Self, Error> {
        drop(left.decompose());
        drop(right.decompose());
        Ok(Self)
    }
}
