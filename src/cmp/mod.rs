mod path;

use crate::{Error, Result};

pub fn dirs(left: &std::path::Path, right: &std::path::Path) -> Result<(path::Dir, path::Dir)> {
    let left_name = path::name(left)?;
    let right_name = path::name(right)?;
    let (left_dirs, left_files) = read_dir(left)?;
    let (right_dirs, right_files) = read_dir(right)?;

    let files = dir_files(left, left_files, right, right_files)?;

    todo!()
}

fn read_dir(dir: &std::path::Path) -> Result<(Vec<std::path::PathBuf>, Vec<String>)> {
    let dir = dir
        .read_dir()
        .map_err(|e| Error::PathUnreadable(dir.to_owned(), e))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|e| Error::PathUnreadable(dir.to_owned(), e))?;

    let mut entries = Vec::with_capacity(dir.len());
    let mut dir_count = 0;

    for entry in dir {
        let path = entry.path();
        let name = String::from(entry.file_name().to_string_lossy());
        if path.is_dir() {
            dir_count += 1;
        }
        entries.push((path, name));
    }

    let mut dirs = Vec::with_capacity(dir_count);
    let mut files = Vec::with_capacity(entries.len() - dir_count);

    for (path, name) in entries {
        if path.is_dir() {
            dirs.push(path);
        } else {
            files.push(name);
        }
    }

    Ok((dirs, files))
}

fn dir_files(
    left_parent: &std::path::Path,
    left: Vec<String>,
    right_parent: &std::path::Path,
    right: Vec<String>,
) -> Result<Vec<path::File>> {
    if left.is_empty() {
        return right
            .into_iter()
            .map(path::name)
            .map(|result| {
                result.map(|name| path::File {
                    name,
                    status: path::Status::MissingLeft,
                })
            })
            .collect();
    }

    if right.is_empty() {
        return left
            .into_iter()
            .map(path::name)
            .map(|result| {
                result.map(|name| path::File {
                    name,
                    status: path::Status::MissingRight,
                })
            })
            .collect();
    }

    let mut names = left;
    names.sort_unstable();

    for name in right
        .into_iter()
        .map(path::name)
        .collect::<Result<Vec<_>>>()?
    {
        if let Err(index) = names.binary_search(&name) {
            names.insert(index, name);
        }
    }

    names
        .into_iter()
        .map(|name| files(left_parent, right_parent, name))
        .collect()
}

pub fn files(left: &std::path::Path, right: &std::path::Path, name: String) -> Result<path::File> {
    use std::io::Read;

    let left = left.join(&name);
    let right = right.join(&name);

    match (left.exists(), right.exists()) {
        (false, false) => Err(Error::TwoMissingFiles(left, right)),
        (true, false) => Ok(path::File {
            name,
            status: path::Status::MissingRight,
        }),
        (false, true) => Ok(path::File {
            name,
            status: path::Status::MissingLeft,
        }),
        (true, true) => {
            let mut left_file = match std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(&left)
            {
                Ok(left) => left,
                Err(e) => return Err(Error::CannotRead(left, e)),
            };

            let mut right_file = match std::fs::OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(&right)
            {
                Ok(right) => right,
                Err(e) => return Err(Error::CannotRead(right, e)),
            };

            let mut left_buffer = [0; 1024 * 4];
            let mut right_buffer = [0; 1024 * 4];

            loop {
                let left_read = match left_file.read(&mut left_buffer) {
                    Ok(r) => r,
                    Err(e) => break Err(Error::CannotRead(left, e)),
                };
                let right_read = match right_file.read(&mut right_buffer) {
                    Ok(r) => r,
                    Err(e) => break Err(Error::CannotRead(right, e)),
                };

                if left_buffer[..left_read] != right_buffer[..right_read] {
                    break Ok(path::File {
                        name,
                        status: path::Status::Partial,
                    });
                }

                if left_read == 0 {
                    break Ok(path::File {
                        name,
                        status: path::Status::Equal,
                    });
                }
            }
        }
    }
}
