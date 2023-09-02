#[derive(Debug)]
pub struct Directory {
    path: std::path::PathBuf,
    entries: Vec<Entry>,
}

impl Directory {
    #[must_use]
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    #[must_use]
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    #[must_use]
    pub fn decompose(self) -> (std::path::PathBuf, Vec<Entry>) {
        (self.path, self.entries)
    }
}

impl Directory {
    pub(crate) fn new(path: std::path::PathBuf, entries: Vec<Entry>) -> Self {
        Self { path, entries }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Entry {
    pub(crate) hash: Hash,
    pub(crate) path: std::path::PathBuf,
    pub(crate) status: Status,
}

impl Entry {
    #[must_use]
    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    #[must_use]
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    #[must_use]
    pub fn status(&self) -> &Status {
        &self.status
    }
}

impl Entry {
    pub(crate) fn new(
        path: &std::path::Path,
        base: &std::path::Path,
        hash: Hash,
    ) -> Result<Self, std::path::StripPrefixError> {
        let path = path.strip_prefix(base).map(std::path::Path::to_path_buf)?;

        Ok(Self {
            hash,
            path,
            status: Status::Unique,
        })
    }
}

impl std::cmp::Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hash.cmp(&other.hash) {
            std::cmp::Ordering::Equal => self.path.cmp(&other.path),
            c => c,
        }
    }
}

impl std::cmp::PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hash([u8; 16]);

impl Hash {
    pub(crate) fn new(array: impl Into<[u8; 16]>) -> Self {
        Self(array.into())
    }

    pub(crate) fn decrement(&self) -> Self {
        let mut hash = self.0;
        for byte in hash.iter_mut().rev().skip_while(|b| **b == 0).take(1) {
            *byte -= 1;
        }
        Self(hash)
    }
}

impl std::fmt::LowerHex for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num = u128::from_be_bytes(self.0);
        std::fmt::LowerHex::fmt(&num, f)
    }
}

impl std::fmt::UpperHex for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num = u128::from_be_bytes(self.0);
        std::fmt::UpperHex::fmt(&num, f)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Status {
    Same(usize),
    Moved(usize),
    Modified(usize),
    Maybe(Vec<usize>),
    Unique,
}

impl Status {
    fn as_index(&self) -> u8 {
        match self {
            Status::Same(_) => 0,
            Status::Moved(_) => 1,
            Status::Modified(_) => 2,
            Status::Maybe(_) => 3,
            Status::Unique => 4,
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Same(_) => f.write_str("SAME"),
            Status::Moved(_) => f.write_str("MOVED"),
            Status::Modified(_) => f.write_str("MODIFIED"),
            Status::Maybe(_) => f.write_str("MAYBE"),
            Status::Unique => f.write_str("UNIQUE"),
        }
    }
}

impl std::cmp::Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_index().cmp(&other.as_index())
    }
}

impl std::cmp::PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
