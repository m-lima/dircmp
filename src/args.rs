#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Expect two directories to compare. Got {0}")]
    MissingParameter(u8),
    #[error("Path does not exist: {0}")]
    PathDoesNotExist(std::path::PathBuf),
    #[error("Path is not a directory: {0}")]
    PathNotDirectory(std::path::PathBuf),
}

// TODO: Update to clap
// TODO: Update verbose to log level and use logging crate
pub fn get() -> Result<(std::path::PathBuf, std::path::PathBuf, bool), Error> {
    let mut args = std::env::args_os().skip(1);

    let left = args
        .next()
        .ok_or(Error::MissingParameter(0))
        .and_then(to_dir_path)?;

    let right = args
        .next()
        .ok_or(Error::MissingParameter(1))
        .and_then(to_dir_path)?;

    Ok((left, right, true))
}

fn to_dir_path(path: std::ffi::OsString) -> Result<std::path::PathBuf, Error> {
    let path = std::path::PathBuf::from(path);

    if !path.exists() {
        Err(Error::PathDoesNotExist(path))
    } else if !path.is_dir() {
        Err(Error::PathNotDirectory(path))
    } else {
        Ok(path)
    }
}
