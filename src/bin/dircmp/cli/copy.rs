#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("There was no parent for `{0}`")]
    NoParent(std::path::PathBuf),
    #[error("Could not create dir `{0}`: {1}")]
    CreateDir(std::path::PathBuf, std::io::Error),
    #[error("Could not copy file `{0}`: {1}")]
    CopyFile(std::path::PathBuf, std::io::Error),
}

pub fn copy(
    reference: dircmp::Directory,
    derived: dircmp::Directory,
    target: &std::path::Path,
) -> Result<usize, Error> {
    let reference = copy_reference(reference, target)?;
    let derived = copy_derived(derived, target)?;
    Ok(reference + derived)
}

fn copy_reference(reference: dircmp::Directory, target: &std::path::Path) -> Result<usize, Error> {
    let mut count = 0;
    let (path, entries) = reference.decompose();

    for entry in entries
        .into_iter()
        .filter(|e| matches!(e.status(), dircmp::Status::Same(_)))
    {
        copy_file("unconflicting", &path, target, entry.path())?;
        count += 1;
    }

    Ok(count)
}

fn copy_derived(derived: dircmp::Directory, target: &std::path::Path) -> Result<usize, Error> {
    let mut count = 0;
    let (path, entries) = derived.decompose();

    for entry in entries {
        let status = match entry.status() {
            dircmp::Status::Moved(_) => "moved",
            dircmp::Status::Maybe(_) => "merged",
            dircmp::Status::Unique => "new",
            _ => continue,
        };

        copy_file(status, &path, target, entry.path())?;
        count += 1;
    }

    Ok(count)
}

fn copy_file(
    status: &'static str,
    src: &std::path::Path,
    dst: &std::path::Path,
    path: &std::path::Path,
) -> Result<(), Error> {
    let target = dst.join(path);
    log::info!("Copying {status} file `{}`", target.display());

    if let Some(dir) = target.parent() {
        if let Err(err) = std::fs::create_dir_all(dir) {
            return Err(Error::CreateDir(std::path::PathBuf::from(dir), err));
        }
    } else {
        return Err(Error::NoParent(target));
    }

    if let Err(err) = std::fs::copy(src.join(path), &target) {
        return Err(Error::CopyFile(target, err));
    }

    Ok(())
}
