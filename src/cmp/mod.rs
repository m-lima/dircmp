mod path;
mod sorted_vec;

use crate::{Error, Result};

pub fn dirs(left: &std::path::Path, right: &std::path::Path) -> Result<path::Status> {
    match (left.exists(), right.exists()) {
        (false, false) => Err(Error::TwoMissingFiles(
            left.to_path_buf(),
            right.to_path_buf(),
        )),
        (true, false) => Ok(path::Status::MissingRight),
        (false, true) => Ok(path::Status::MissingLeft),
        (true, true) => {
            let (left_dirs, left_files) = split_dir(left)?;
            let (right_dirs, right_files) = split_dir(right)?;

            let files = dir_files(left_files, right_files)?;
            let dirs = dir_dirs(left_dirs, right_dirs)?;

            let mut children = Vec::with_capacity(files.len() + dirs.len());

            children.extend(files.into_iter().map(path::Path::File));
            children.extend(dirs.into_iter().map(path::Path::Dir));

            todo!()
        }
    }
}

pub fn files(left: &std::path::Path, right: &std::path::Path) -> Result<path::Status> {
    use std::io::Read;

    match (left.exists(), right.exists()) {
        (false, false) => Err(Error::TwoMissingFiles(
            left.to_path_buf(),
            right.to_path_buf(),
        )),
        (true, false) => Ok(path::Status::MissingRight),
        (false, true) => Ok(path::Status::MissingLeft),
        (true, true) => {
            let mut left_file = match std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(left)
            {
                Ok(left) => left,
                Err(e) => return Err(Error::CannotRead(left.to_path_buf(), e)),
            };

            let mut right_file = match std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(right)
            {
                Ok(right) => right,
                Err(e) => return Err(Error::CannotRead(right.to_path_buf(), e)),
            };

            let mut left_buffer = [0; 1024 * 4];
            let mut right_buffer = [0; 1024 * 4];

            loop {
                let left_read = match left_file.read(&mut left_buffer) {
                    Ok(r) => r,
                    Err(e) => break Err(Error::CannotRead(left.to_path_buf(), e)),
                };
                let right_read = match right_file.read(&mut right_buffer) {
                    Ok(r) => r,
                    Err(e) => break Err(Error::CannotRead(right.to_path_buf(), e)),
                };

                if left_buffer[..left_read] != right_buffer[..right_read] {
                    break Ok(path::Status::Partial);
                }

                if left_read == 0 {
                    break Ok(path::Status::Equal);
                }
            }
        }
    }
}

fn dirs_internal(
    left: &std::path::Path,
    right: &std::path::Path,
    name: std::ffi::OsString,
) -> Result<path::Dir> {
    dirs(&left.join(&name), &right.join(&name)).map(|status| path::Dir {
        name: path::stringify(name),
        status,
        children: Vec::new(),
    })
}

fn files_internal(
    left: &std::path::Path,
    right: &std::path::Path,
    name: std::ffi::OsString,
) -> Result<path::File> {
    files(&left.join(&name), &right.join(&name)).map(|status| path::File {
        name: path::stringify(name),
        status,
    })
}

fn split_dir(parent: &std::path::Path) -> Result<(path::Paths, path::Paths)> {
    let dir = parent
        .read_dir()
        .map_err(|e| Error::PathUnreadable(parent.to_path_buf(), e))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|e| Error::PathUnreadable(parent.to_path_buf(), e))?;

    let mut entries = Vec::with_capacity(dir.len());
    let mut dir_count = 0;

    for entry in dir {
        let path = entry.path();
        let name = entry.file_name();
        if path.is_dir() {
            dir_count += 1;
        }
        entries.push((path, name));
    }

    let mut dirs = Vec::with_capacity(dir_count);
    let mut files = Vec::with_capacity(entries.len() - dir_count);

    for (path, name) in entries {
        if path.is_dir() {
            dirs.push(name);
        } else {
            files.push(name);
        }
    }

    let dirs = path::Paths {
        parent: std::path::PathBuf::from(parent),
        paths: dirs,
    };

    let files = path::Paths {
        parent: std::path::PathBuf::from(parent),
        paths: files,
    };

    Ok((dirs, files))
}

fn dir_files(
    prefix: std::path::PathBuf,
    left: path::Paths,
    right: path::Paths,
) -> Result<Vec<path::File>> {
    if left.paths.is_empty() {
        return Ok(make_missing(right, |name| path::File {
            name,
            status: path::Status::MissingLeft,
        }));
    }

    if right.paths.is_empty() {
        return Ok(make_missing(left, |name| path::File {
            name,
            status: path::Status::MissingRight,
        }));
    }

    let left_files = sorted_vec::SortedVec::new(left.paths);
    let right_files = sorted_vec::SortedVec::new(right.paths);
    let names = left_files.merge(right_files);

    names
        .to_iter()
        .map(|name| files_internal(&left.parent, &right.parent, name))
        .collect()
}

fn dir_dirs(left: path::Paths, right: path::Paths) -> Result<Vec<path::Dir>> {
    if left.paths.is_empty() {
        return Ok(make_missing(right, |name| path::Dir {
            name,
            status: path::Status::MissingLeft,
            // TODO: Populate these guys
            children: Vec::new(),
        }));
    }

    if right.paths.is_empty() {
        return Ok(make_missing(left, |name| path::Dir {
            name,
            status: path::Status::MissingRight,
            // TODO: Populate these guys
            children: Vec::new(),
        }));
    }

    let left_files = sorted_vec::SortedVec::new(left.paths);
    let right_files = sorted_vec::SortedVec::new(right.paths);
    let names = left_files.merge(right_files);

    names
        .to_iter()
        .map(|name| dirs_internal(&left.parent, &right.parent, name))
        .collect()
}

fn make_missing<T, F: Fn(String) -> T>(paths: path::Paths, f: F) -> Vec<T> {
    paths
        .paths
        .into_iter()
        .map(path::stringify)
        .map(f)
        .collect()
}
