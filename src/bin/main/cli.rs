use super::args;

pub fn run(args: args::Args) -> std::process::ExitCode {
    let start = std::time::Instant::now();

    if let Err(e) = init_logger(args.verbosity()) {
        eprintln!("[31merror:[m {e}");
        return std::process::ExitCode::FAILURE;
    }

    if let Err(e) = fallible_run(args.left, args.right) {
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

fn fallible_run(left: std::path::PathBuf, right: std::path::PathBuf) -> Result<(), dircmp::Error> {
    let (left, right) = dircmp::compare(left, right)?;

    print(&left, &right, false);
    print(&right, &left, true);
    Ok(())
}

fn print(reference: &dircmp::Index, other: &dircmp::Index, skip: bool) {
    println!("{}", reference.path().display());
    for index in reference.children() {
        match index.status() {
            dircmp::Status::Same(_) => {
                if !skip {
                    println!("[32mMATCHES[m");
                    println!("[32m└[m {}", index.path().display());
                }
            }
            dircmp::Status::Moved(i) => {
                if !skip {
                    println!("[33mMOVED");
                    println!("[33m┌[m {}", index.path().display());
                    println!("[33m└[m {}", unsafe {
                        other.children().get_unchecked(*i).path().display()
                    });
                }
            }
            dircmp::Status::Hash(indices) => {
                let Some((tail, head)) = indices.split_last() else {
                    continue;
                };
                println!("[34mMAYBE");
                println!("[34m╱[m {}", index.path().display());
                for i in head {
                    println!("[34m├[m {}", unsafe {
                        other.children().get_unchecked(*i).path().display()
                    });
                }
                println!("[34m└[m {}", unsafe {
                    other.children().get_unchecked(*tail).path().display()
                });
            }
            dircmp::Status::Unique => {
                println!("[31mUNIQUE[m");
                println!("[31m└[m {}", index.path().display());
            }
        }
    }
}
