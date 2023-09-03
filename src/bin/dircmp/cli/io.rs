type Dirs = (dircmp::Directory, dircmp::Directory);
type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not write entries: {0}")]
    Write(bincode::Error),
    #[error("Could not read entries: {0}")]
    Read(bincode::Error),
    #[error("Could not write to summary: {0}")]
    Summary(std::io::Error),
    #[error("Could not write to stdout: {0}")]
    Print(std::io::Error),
}

pub fn from_binary(input: impl std::io::Read) -> Result<Dirs> {
    let start = std::time::Instant::now();

    log::info!("Reading from input file");
    let reader = std::io::BufReader::new(input);
    let dirs = bincode::deserialize_from(reader).map_err(Error::Read)?;
    log::info!("Finished reading from input file in {:?}", start.elapsed());

    Ok(dirs)
}

pub fn to_binary(output: impl std::io::Write, dirs: &Dirs) -> Result {
    let start = std::time::Instant::now();

    let writer = std::io::BufWriter::new(output);
    log::info!("Writing to output file");
    bincode::serialize_into(writer, dirs).map_err(Error::Write)?;
    log::info!("Finished writing to output file in {:?}", start.elapsed());

    Ok(())
}

pub fn to_summary(output: impl std::io::Write, dirs: &Dirs) -> Result {
    let start = std::time::Instant::now();

    let mut writer = std::io::BufWriter::new(output);
    log::info!("Writing summary");
    write_tsv(&mut writer, dirs, Mode::Left).map_err(Error::Summary)?;
    write_tsv(&mut writer, dirs, Mode::Right).map_err(Error::Summary)?;
    log::info!("Finished writing summary in {:?}", start.elapsed());

    Ok(())
}

pub fn to_stdout(dirs: &Dirs, show_matched: bool) -> Result {
    write_pretty(dirs, show_matched, Mode::Left).map_err(Error::Print)?;
    write_pretty(dirs, show_matched, Mode::Right).map_err(Error::Print)
}

fn write_tsv(mut out: impl std::io::Write, dirs: &Dirs, mode: Mode) -> std::io::Result<()> {
    let (reference, other) = match mode {
        Mode::Left => (&dirs.0, &dirs.1),
        Mode::Right => (&dirs.1, &dirs.0),
    };

    for entry in reference.entries() {
        match entry.status() {
            dircmp::Status::Same(_) => {}
            status @ (dircmp::Status::Moved(i) | dircmp::Status::Modified(i)) => {
                if mode == Mode::Left {
                    writeln!(
                        out,
                        "{path}	{status}	{other}",
                        path = reference.path().join(entry.path()).display(),
                        other = other
                            .path()
                            .join(unsafe { other.entries().get_unchecked(*i).path() })
                            .display()
                    )?;
                }
            }
            status @ dircmp::Status::Maybe(indices) => {
                write!(
                    out,
                    "{path}	{status}",
                    path = reference.path().join(entry.path()).display()
                )?;
                for i in indices {
                    write!(
                        out,
                        "	{path}",
                        path = other
                            .path()
                            .join(unsafe { other.entries().get_unchecked(*i).path() })
                            .display()
                    )?;
                }
                writeln!(out)?;
            }
            status @ (dircmp::Status::Unique | dircmp::Status::Empty) => {
                writeln!(
                    out,
                    "{path}	{status}",
                    path = reference.path().join(entry.path()).display(),
                )?;
            }
        }
    }

    Ok(())
}

fn write_pretty(dirs: &Dirs, show_matched: bool, mode: Mode) -> std::io::Result<()> {
    use std::io::Write;

    let (reference, other) = match mode {
        Mode::Left => (&dirs.0, &dirs.1),
        Mode::Right => (&dirs.1, &dirs.0),
    };

    let mut out = std::io::stdout().lock();
    writeln!(out, "[37mVisiting:[m {}", reference.path().display())?;
    for entry in reference.entries() {
        match entry.status() {
            status @ dircmp::Status::Same(_) => {
                if show_matched && mode == Mode::Left {
                    writeln!(out, "[32m{mode} {status:<8}[m {}", entry.path().display())?;
                }
            }
            status @ dircmp::Status::Moved(i) => {
                if mode == Mode::Left {
                    writeln!(out, "[33m{mode} {status:<8}[m {}", entry.path().display())?;
                    writeln!(out, "[33m  └[m {}", unsafe {
                        other.entries().get_unchecked(*i).path().display()
                    })?;
                }
            }
            status @ dircmp::Status::Modified(i) => {
                if mode == Mode::Left {
                    writeln!(out, "[35m{mode} {status:<8}[m {}", entry.path().display())?;
                    writeln!(out, "[35m  └[m {}", unsafe {
                        other.entries().get_unchecked(*i).path().display()
                    })?;
                }
            }
            status @ dircmp::Status::Maybe(indices) => {
                let Some((tail, head)) = indices.split_last() else {
                    continue;
                };
                writeln!(out, "[34m{mode} {status:<8}[m {}", entry.path().display())?;
                for i in head {
                    writeln!(out, "[34m  ├[m {}", unsafe {
                        other.entries().get_unchecked(*i).path().display()
                    })?;
                }
                writeln!(out, "[34m  └[m {}", unsafe {
                    other.entries().get_unchecked(*tail).path().display()
                })?;
            }
            status @ dircmp::Status::Unique => {
                writeln!(out, "[31m{mode} {status:<8}[m {}", entry.path().display())?;
            }
            status @ dircmp::Status::Empty => {
                writeln!(out, "[36m{mode} {status:<8}[m {}", entry.path().display())?;
            }
        }
    }

    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Mode {
    Left,
    Right,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Left => f.write_str("<"),
            Mode::Right => f.write_str(">"),
        }
    }
}
