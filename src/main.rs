type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Expect two directories to compare. Got {0}")]
    MissingParameter(u8),
    #[error("Path does not exist: {0}")]
    PathDoesNotExist(String),
    #[error("Path is not a directory: {0}")]
    PathNotDirectory(String),
    #[error("Could not read path `{0}`: {1}")]
    PathUnreadable(std::path::PathBuf, std::io::Error),
    #[error("Failed to write to stdout: {0}")]
    Io(#[from] std::io::Error),
}

fn get_params() -> Result<(String, String)> {
    let mut args = std::env::args().skip(1);
    let left = args.next().ok_or_else(|| Error::MissingParameter(0))?;
    let right = args.next().ok_or_else(|| Error::MissingParameter(1))?;
    Ok((left, right))
}

fn to_dir_reader(string: String) -> Result<std::path::PathBuf> {
    let path = std::path::PathBuf::from(&string);

    if !path.exists() {
        Err(Error::PathDoesNotExist(string))
    } else if !path.is_dir() {
        Err(Error::PathNotDirectory(string))
    } else {
        Ok(path)
    }
}

fn read_dir(dir: &std::path::Path) -> Result<(Vec<std::path::PathBuf>, Vec<std::path::PathBuf>)> {
    let left = dir
        .read_dir()
        .map_err(|e| Error::PathUnreadable(dir.to_owned(), e))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|e| Error::PathUnreadable(dir.to_owned(), e))?;
    let (mut dirs, files) = left
        .into_iter()
        .map(|entry| entry.path())
        .partition::<Vec<_>, _>(|entry| entry.is_dir());
    dirs.shrink_to_fit();

    Ok((dirs, files))
}

fn normalize_files(files: Vec<std::path::PathBuf>) -> Vec<std::ffi::OsString> {
    let mut files = files
        .into_iter()
        .filter_map(|p| p.file_name().map(std::ffi::OsStr::to_os_string))
        .collect::<Vec<_>>();
    files.sort_unstable();
    files
}

fn compare_files(
    left: Vec<std::path::PathBuf>,
    right: Vec<std::path::PathBuf>,
    out: &mut impl std::io::Write,
) -> Result {
    let left = normalize_files(left);
    let right = normalize_files(right);

    write!(out, "[33m")?;
    for file in &left {
        if right.binary_search(file).is_err() {
            writeln!(out, "< {file:?}")?;
        }
    }

    write!(out, "[34m")?;
    for file in &right {
        if left.binary_search(file).is_err() {
            writeln!(out, "> {file:?}")?;
        }
    }
    write!(out, "[m")?;
    out.flush()?;
    Ok(())
}

fn compare_dirs(
    left: &std::path::Path,
    right: &std::path::Path,
    out: &mut impl std::io::Write,
) -> Result {
    let (left_dirs, mut left_files) = read_dir(left)?;
    let (right_dirs, mut right_files) = read_dir(right)?;

    writeln!(out, "Comparing")?;
    writeln!(out, "[33m{:?}", left.as_os_str(),)?;
    writeln!(out, "[34m{:?}", right.as_os_str())?;
    writeln!(out, "[m------")?;
    compare_files(left_files, right_files, out)?;
    Ok(())
}

fn fallible_main() -> Result {
    let (left, right) = get_params()?;
    let left = to_dir_reader(left)?;
    let right = to_dir_reader(right)?;
    let mut out = std::io::stdout().lock();
    compare_dirs(&left, &right, &mut out)
}

fn main() -> std::process::ExitCode {
    if let Err(e) = fallible_main() {
        eprintln!("[31mERROR[m {e}");
        1
    } else {
        0
    }
    .into()
}
