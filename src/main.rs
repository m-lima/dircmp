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
    #[error("Inconsistent mutal directories found")]
    InconsistentSize,
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
    let (dirs, files) = left
        .into_iter()
        .map(|entry| entry.path())
        .partition::<Vec<_>, _>(|entry| entry.is_dir());

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

fn normalize_dirs(files: Vec<std::path::PathBuf>) -> Vec<(std::ffi::OsString, std::path::PathBuf)> {
    let mut files = files
        .into_iter()
        .filter_map(|p| p.file_name().map(|name| (name.to_os_string(), p.clone())))
        .collect::<Vec<_>>();
    files.sort_unstable();
    files
}

fn print_diff(
    left: Vec<std::path::PathBuf>,
    right: Vec<std::path::PathBuf>,
    out: &mut impl std::io::Write,
) -> Result {
    let left = normalize_files(left);
    let right = normalize_files(right);

    write!(out, "[33m")?;
    for file in &left {
        if right.binary_search(file).is_err() {
            writeln!(out, "< {}", file.to_string_lossy())?;
        }
    }

    write!(out, "[34m")?;
    for file in &right {
        if left.binary_search(file).is_err() {
            writeln!(out, "> {}", file.to_string_lossy())?;
        }
    }
    write!(out, "[m")?;
    out.flush()?;
    Ok(())
}

fn print_diff_and_get_mutual(
    left: Vec<std::path::PathBuf>,
    right: Vec<std::path::PathBuf>,
    out: &mut impl std::io::Write,
) -> Result<Vec<(std::path::PathBuf, std::path::PathBuf)>> {
    let left = normalize_dirs(left);
    let right = normalize_dirs(right);
    let mut left_mutual = Vec::new();
    let mut right_mutual = Vec::new();

    write!(out, "[33m")?;
    for file in &left {
        if right
            .binary_search_by(|(name, _)| name.cmp(&file.0))
            .is_err()
        {
            writeln!(out, "< {}", file.0.to_string_lossy())?;
        } else {
            left_mutual.push(file.1.clone());
        }
    }

    write!(out, "[34m")?;
    for file in right {
        if left
            .binary_search_by(|(name, _)| name.cmp(&file.0))
            .is_err()
        {
            writeln!(out, "> {}", file.0.to_string_lossy())?;
        } else {
            right_mutual.push(file.1);
        }
    }
    write!(out, "[m")?;
    out.flush()?;

    if left_mutual.len() == right_mutual.len() {
        let mut mutual = left_mutual
            .into_iter()
            .zip(right_mutual.into_iter())
            .collect::<Vec<_>>();
        mutual.shrink_to_fit();
        Ok(mutual)
    } else {
        Err(Error::InconsistentSize)
    }
}

fn compare_dirs(
    left: &std::path::Path,
    right: &std::path::Path,
    out: &mut impl std::io::Write,
) -> Result {
    let (left_dirs, left_files) = read_dir(left)?;
    let (right_dirs, right_files) = read_dir(right)?;

    writeln!(out, "Comparing")?;
    writeln!(out, "[33m{}", left.display())?;
    writeln!(out, "[34m{}", right.display())?;
    writeln!(out, "[m------")?;
    print_diff(left_files, right_files, out)?;
    let mutual = print_diff_and_get_mutual(left_dirs, right_dirs, out)?;

    for (left, right) in mutual {
        compare_dirs(&left, &right, out)?;
    }

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
