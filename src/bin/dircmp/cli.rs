use super::args;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Dircmp(#[from] dircmp::Error),
    #[error("Could not write entries: {0}")]
    Write(serde_yaml::Error),
    #[error("Could not read entries: {0}")]
    Read(serde_yaml::Error),
    #[error("Could not write to stdout: {0}")]
    Print(#[from] std::io::Error),
}

pub fn run(args: args::Command) -> std::process::ExitCode {
    if let Err(e) = init_logger(args.verbosity()) {
        eprintln!("[31merror:[m {e}");
        return std::process::ExitCode::FAILURE;
    }

    match args {
        args::Command::Scan(args) => {
            let start = std::time::Instant::now();

            if let Err(e) = scan(args.left, args.right, args.output, args.print) {
                log::error!("{e}");
                return std::process::ExitCode::FAILURE;
            }

            log::info!("Elapsed: {:?}", start.elapsed());
        }
        args::Command::Print(args) => {
            if let Err(e) = print(&args.input, args.matched) {
                log::error!("{e}");
                return std::process::ExitCode::FAILURE;
            }
        }
    }

    std::process::ExitCode::SUCCESS
}

fn init_logger(level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    simplelog::TermLogger::init(
        level,
        match simplelog::ConfigBuilder::default()
            .set_target_level(log::LevelFilter::Error)
            .set_thread_level(log::LevelFilter::Off)
            .set_time_offset_to_local()
        {
            Ok(b) => b.set_time_format_custom(simplelog::format_description!(
                "[year]-[month]-[day]T[hour]:[minute]:[second]"
            )),
            Err(b) => b.set_time_format_custom(simplelog::format_description!(
                "[year]-[month]-[day]T[hour]:[minute]:[second]Z"
            )),
        }
        .build(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )?;
    log::info!("Log level set to {level}");
    Ok(())
}

fn scan(
    left: std::path::PathBuf,
    right: std::path::PathBuf,
    output: Option<std::sync::Arc<std::fs::File>>,
    print_filter: args::PrintFilter,
) -> Result<(), Error> {
    log::debug!(
        "left: {left}, right: {right}, output: {output}, print_filter: {print_filter:?}",
        left = left.display(),
        right = right.display(),
        output = output.is_some(),
    );

    let result = dircmp::compare(left, right)?;

    if let Some(output) = output {
        let start = std::time::Instant::now();
        log::info!("Writing to output file");
        serde_yaml::to_writer(output.as_ref(), &result).map_err(Error::Write)?;
        log::info!("Finished writing to output file in {:?}", start.elapsed());
    }

    match print_filter {
        args::PrintFilter::None => {}
        filter => {
            let (left, right) = result;
            write(&left, &right, filter == args::PrintFilter::All, Mode::Left)?;
            write(&right, &left, filter == args::PrintFilter::All, Mode::Right)?;
        }
    }
    Ok(())
}

fn print(input: &std::fs::File, show_matched: bool) -> Result<(), Error> {
    let (left, right) = serde_yaml::from_reader(input).map_err(Error::Read)?;

    write(&left, &right, show_matched, Mode::Left)?;
    write(&right, &left, show_matched, Mode::Right)?;

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

fn write(
    reference: &dircmp::Directory,
    other: &dircmp::Directory,
    show_matched: bool,
    mode: Mode,
) -> std::io::Result<()> {
    use std::io::Write;

    let mut out = std::io::stdout().lock();
    writeln!(out, "[37mVisiting:[m {}", reference.path().display())?;
    for index in reference.entries() {
        match index.status() {
            status @ dircmp::Status::Same(_) => {
                if show_matched && mode == Mode::Left {
                    writeln!(out, "[32m{mode} {status:<8}[m {}", index.path().display())?;
                }
            }
            status @ dircmp::Status::Moved(i) => {
                if mode == Mode::Left {
                    writeln!(out, "[33m{mode} {status:<8}[m {}", index.path().display())?;
                    writeln!(out, "[33m  └[m {}", unsafe {
                        other.entries().get_unchecked(*i).path().display()
                    })?;
                }
            }
            status @ dircmp::Status::Modified(i) => {
                if mode == Mode::Left {
                    writeln!(out, "[35m{mode} {status:<8}[m {}", index.path().display())?;
                    writeln!(out, "[35m  └[m {}", unsafe {
                        other.entries().get_unchecked(*i).path().display()
                    })?;
                }
            }
            status @ dircmp::Status::Maybe(indices) => {
                let Some((tail, head)) = indices.split_last() else {
                    continue;
                };
                writeln!(out, "[34m{mode} {status:<8}[m {}", index.path().display())?;
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
                writeln!(out, "[31m{mode} {status:<8}[m {}", index.path().display())?;
            }
        }
    }

    Ok(())
}
