mod copy;
mod io;

use super::args;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Dircmp(#[from] dircmp::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Copy(#[from] copy::Error),
}

pub fn run(args: args::Command) -> std::process::ExitCode {
    if let Err(e) = init_logger(args.verbosity()) {
        eprintln!("[31merror:[m {e}");
        return std::process::ExitCode::FAILURE;
    }

    let start = std::time::Instant::now();
    if let Err(e) = match args {
        args::Command::Scan(args) => scan(args),
        args::Command::Print(args) => print(args),
        args::Command::Copy(args) => copy(args),
    } {
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

fn scan(
    args::Scan {
        print: print_filter,
        left,
        right,
        output,
        summary,
        verbosity: _,
    }: args::Scan,
) -> Result<(), Error> {
    log::debug!(
        "left: {left}, right: {right}, output: {output}, summary: {summary}, print_filter: {print_filter:?}",
        left = left.display(),
        right = right.display(),
        output = output.is_some(),
        summary = summary.is_some(),
    );

    let dirs = dircmp::compare(left, right)?;

    if let Some(output) = output {
        io::to_binary(output.as_ref(), &dirs)?;
    }

    if let Some(output) = summary {
        io::to_summary(output.as_ref(), &dirs)?;
    }

    let show_matched = match print_filter {
        args::PrintFilter::None => return Ok(()),
        args::PrintFilter::Diff => false,
        args::PrintFilter::All => true,
    };
    io::to_stdout(&dirs, show_matched)?;
    Ok(())
}

fn print(
    args::Print {
        input,
        summary,
        matched: show_matched,
        verbosity: _,
    }: args::Print,
) -> Result<(), Error> {
    log::debug!(
        "summary: {summary}, show_matched: {show_matched}",
        summary = summary.is_some(),
    );

    let dirs = io::from_binary(input.as_ref())?;

    if let Some(output) = summary {
        io::to_summary(output.as_ref(), &dirs)?;
    }

    io::to_stdout(&dirs, show_matched)?;

    Ok(())
}

fn copy(
    args::Copy {
        verbosity: _,
        reference,
        derived,
        target,
    }: args::Copy,
) -> Result<(), Error> {
    log::debug!(
        "reference: {reference}, derived: {derived}, target: {target}",
        reference = reference.display(),
        derived = derived.display(),
        target = target.display(),
    );

    let (reference, derived) = dircmp::compare(reference, derived)?;

    let start = std::time::Instant::now();
    let entries = copy::copy(reference, derived, &target)?;
    log::info!(
        "Finished copying {entries} files into {} in {:?}",
        target.display(),
        start.elapsed(),
    );

    Ok(())
}
