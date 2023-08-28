mod args;
mod error;
mod hasher;
mod thread;

fn main() -> std::process::ExitCode {
    let start = std::time::Instant::now();
    let args = args::parse();

    if let Err(e) = init_logger(args.verbosity()) {
        eprintln!("[31merror:[m {e}");
        return std::process::ExitCode::FAILURE;
    }

    if let Err(e) = fallible_main(args) {
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

fn fallible_main(args: args::Args) -> error::Result {
    let pool = thread::pool()?;
    let left = hasher::Index::new(args.left, &pool)?;
    let right = hasher::Index::new(args.right, &pool)?;
    log::info!("{}: {}", left.path().display(), left.len());
    log::info!("{}: {}", right.path().display(), right.len());
    Ok(())
}
