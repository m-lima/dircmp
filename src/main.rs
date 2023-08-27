mod cmp;
mod error;

use error::{Error, Result};

fn main() -> std::process::ExitCode {
    if let Err(e) = fallible_main() {
        eprintln!("[31mERROR[m {e}");
        1
    } else {
        0
    }
    .into()
}

fn fallible_main() -> Result {
    let (left, right) = get_params()?;
    cmp::dirs(&left, &right)?;
    Ok(())
}

fn get_params() -> Result<(std::path::PathBuf, std::path::PathBuf)> {
    let mut args = std::env::args().skip(1);

    let left = args
        .next()
        .ok_or_else(|| Error::MissingParameter(0))
        .and_then(to_dir_path)?;

    let right = args
        .next()
        .ok_or_else(|| Error::MissingParameter(1))
        .and_then(to_dir_path)?;

    Ok((left, right))
}

fn to_dir_path(path: String) -> Result<std::path::PathBuf> {
    let path = std::path::PathBuf::from(path);

    if !path.exists() {
        Err(Error::PathDoesNotExist(path))
    } else if !path.is_dir() {
        Err(Error::PathNotDirectory(path))
    } else {
        Ok(path)
    }
}

// fn to_dir_reader(string: String) -> Result<std::path::PathBuf> {
//     let path = std::path::PathBuf::from(&string);
//
//     if !path.exists() {
//         Err(Error::PathDoesNotExist(string))
//     } else if !path.is_dir() {
//         Err(Error::PathNotDirectory(string))
//     } else {
//         Ok(path)
//     }
// }
//
// fn read_dir(dir: &std::path::Path) -> Result<(Vec<std::path::PathBuf>, Vec<std::path::PathBuf>)> {
//     let left = dir
//         .read_dir()
//         .map_err(|e| Error::PathUnreadable(dir.to_owned(), e))?
//         .collect::<std::result::Result<Vec<_>, _>>()
//         .map_err(|e| Error::PathUnreadable(dir.to_owned(), e))?;
//     let (dirs, files) = left
//         .into_iter()
//         .map(|entry| entry.path())
//         .partition::<Vec<_>, _>(|entry| entry.is_dir());
//
//     Ok((dirs, files))
// }
//
// fn sort_path_names(files: Vec<std::path::PathBuf>) -> Vec<NamedPath> {
//     let mut files = files
//         .into_iter()
//         .filter_map(|p| {
//             p.file_name().map(|name| NamedPath {
//                 name: name.to_os_string(),
//                 path: p.clone(),
//             })
//         })
//         .collect::<Vec<_>>();
//     files.sort_unstable();
//     files
// }
//
// fn compare_files(left: &NamedPath, right: &NamedPath, out: &mut impl std::io::Write) -> Result {
//     use std::io::Read;
//
//     let mut left_file = std::fs::OpenOptions::new()
//         .read(true)
//         .write(false)
//         .create(false)
//         .open(&left.path)
//         .map_err(|e| Error::CannotRead(left.path.clone(), e))?;
//
//     let mut right_file = std::fs::OpenOptions::new()
//         .read(true)
//         .write(false)
//         .create(false)
//         .open(&right.path)
//         .map_err(|e| Error::CannotRead(right.path.clone(), e))?;
//
//     let mut left_buffer = [0; 1024 * 4];
//     let mut right_buffer = [0; 1024 * 4];
//
//     loop {
//         let left_read = left_file
//             .read(&mut left_buffer)
//             .map_err(|e| Error::CannotRead(left.path.clone(), e))?;
//         let right_read = right_file
//             .read(&mut right_buffer)
//             .map_err(|e| Error::CannotRead(right.path.clone(), e))?;
//
//         if left_buffer[..left_read] != right_buffer[..right_read] {
//             writeln!(out, "[31m# Mismatch {left} <> {right}[m",)?;
//             break Ok(());
//         }
//
//         if left_read == 0 {
//             break Ok(());
//         }
//     }
// }
//
// fn print_diff(
//     left: Vec<std::path::PathBuf>,
//     right: Vec<std::path::PathBuf>,
//     out: &mut impl std::io::Write,
// ) -> Result {
//     let left = sort_path_names(left);
//     let right = sort_path_names(right);
//
//     write!(out, "[33m")?;
//     for file in &left {
//         if right.binary_search(file).is_err() {
//             writeln!(out, "< {file}")?;
//         }
//     }
//
//     write!(out, "[34m")?;
//     for file in &right {
//         if let Ok(index) = left.binary_search(file) {
//             let left = unsafe { left.get_unchecked(index) };
//             compare_files(left, file, out)?;
//         } else {
//             writeln!(out, "> {file}")?;
//         }
//     }
//
//     write!(out, "[m")?;
//     out.flush()?;
//     Ok(())
// }
//
// fn print_diff_and_get_mutual(
//     left: Vec<std::path::PathBuf>,
//     right: Vec<std::path::PathBuf>,
//     out: &mut impl std::io::Write,
// ) -> Result<Vec<(std::path::PathBuf, std::path::PathBuf)>> {
//     let left = sort_path_names(left);
//     let right = sort_path_names(right);
//     let mut left_mutual = Vec::new();
//     let mut right_mutual = Vec::new();
//
//     write!(out, "[33m")?;
//     for dir in &left {
//         if right.binary_search(dir).is_err() {
//             writeln!(out, "< {dir}")?;
//         } else {
//             left_mutual.push(dir.path.clone());
//         }
//     }
//
//     write!(out, "[34m")?;
//     for dir in right {
//         if left.binary_search(&dir).is_err() {
//             writeln!(out, "> {dir}")?;
//         } else {
//             right_mutual.push(dir.path);
//         }
//     }
//     write!(out, "[m")?;
//     out.flush()?;
//
//     if left_mutual.len() == right_mutual.len() {
//         let mut mutual = left_mutual
//             .into_iter()
//             .zip(right_mutual.into_iter())
//             .collect::<Vec<_>>();
//         mutual.shrink_to_fit();
//         Ok(mutual)
//     } else {
//         Err(Error::InconsistentSize)
//     }
// }
//
// fn compare_dirs(
//     left: &std::path::Path,
//     right: &std::path::Path,
//     out: &mut impl std::io::Write,
// ) -> Result {
//     let (left_dirs, left_files) = read_dir(left)?;
//     let (right_dirs, right_files) = read_dir(right)?;
//
//     writeln!(out, "Comparing")?;
//     writeln!(out, "[33m{}", left.display())?;
//     writeln!(out, "[34m{}", right.display())?;
//     writeln!(out, "[m------")?;
//     print_diff(left_files, right_files, out)?;
//     let mutual = print_diff_and_get_mutual(left_dirs, right_dirs, out)?;
//
//     for (left, right) in mutual {
//         compare_dirs(&left, &right, out)?;
//     }
//
//     Ok(())
// }
//
// fn fallible_main() -> Result {
//     let (left, right) = get_params()?;
//     let left = to_dir_reader(left)?;
//     let right = to_dir_reader(right)?;
//     let mut out = std::io::stdout().lock();
//     compare_dirs(&left, &right, &mut out)
// }
//
// fn main() -> std::process::ExitCode {
//     if let Err(e) = fallible_main() {
//         eprintln!("[31mERROR[m {e}");
//         1
//     } else {
//         0
//     }
//     .into()
// }
