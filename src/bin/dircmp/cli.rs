use super::args;

pub fn run(args: args::Args) -> std::process::ExitCode {
    let start = std::time::Instant::now();

    if let Err(e) = init_logger(args.verbosity()) {
        eprintln!("[31merror:[m {e}");
        return std::process::ExitCode::FAILURE;
    }

    if let Err(e) = fallible_run(args.left, args.right, args.matched) {
        log::error!("{e}");
        return std::process::ExitCode::FAILURE;
    }

    log::info!("Elapsed: {:?}", start.elapsed());
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

fn fallible_run(
    left: std::path::PathBuf,
    right: std::path::PathBuf,
    show_matched: bool,
) -> Result<(), dircmp::Error> {
    log::debug!(
        "left: {left}, right: {right}, show_matched: {show_matched}",
        left = left.display(),
        right = right.display(),
    );
    let (left, right) = dircmp::compare(left, right)?;

    print(&left, &right, show_matched, false);
    print(&right, &left, show_matched, true);
    Ok(())
}

fn print(reference: &dircmp::Index, other: &dircmp::Index, show_matched: bool, skip: bool) {
    println!("[37mVisiting:[m {}", reference.path().display());
    for index in reference.entries() {
        match index.status() {
            dircmp::Status::Same(_) => {
                if show_matched && !skip {
                    println!("[32mMATCHES[m  {}", index.path().display());
                }
            }
            dircmp::Status::Moved(i) => {
                if !skip {
                    println!("[33mMOVED[m    {}", index.path().display());
                    println!("[33m└[m {}", unsafe {
                        other.entries().get_unchecked(*i).path().display()
                    });
                }
            }
            dircmp::Status::Modified(i) => {
                if !skip {
                    println!("[35mMODIFIED[m {}", index.path().display());
                    println!("[35m└[m {}", unsafe {
                        other.entries().get_unchecked(*i).path().display()
                    });
                }
            }
            dircmp::Status::Maybe(indices) => {
                let Some((tail, head)) = indices.split_last() else {
                    continue;
                };
                println!("[34mMAYBE[m    {}", index.path().display());
                for i in head {
                    println!("[34m├[m {}", unsafe {
                        other.entries().get_unchecked(*i).path().display()
                    });
                }
                println!("[34m└[m {}", unsafe {
                    other.entries().get_unchecked(*tail).path().display()
                });
            }
            dircmp::Status::Unique => {
                println!("[31mUNIQUE[m   {}", index.path().display());
            }
        }
    }
}
