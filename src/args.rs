pub fn parse() -> Args {
    <Args as structopt::StructOpt>::from_args()
}

#[derive(Debug, structopt::StructOpt)]
pub struct Args {
    /// Verbosity level
    #[structopt(short, parse(from_occurrences = to_verbosity))]
    pub verbosity: log::LevelFilter,
    /// Path to the `left` directory to compare
    #[structopt(parse(try_from_os_str = parse_dir))]
    pub left: std::path::PathBuf,
    /// Path to the `right` directory to compare
    #[structopt(parse(try_from_os_str = parse_dir))]
    pub right: std::path::PathBuf,
}

fn parse_dir(input: &std::ffi::OsStr) -> Result<std::path::PathBuf, std::ffi::OsString> {
    let path = std::path::PathBuf::from(input);

    if !path.exists() {
        Err(std::ffi::OsString::from("Path does not exist"))
    } else if !path.is_dir() {
        Err(std::ffi::OsString::from("Path is not a directory"))
    } else {
        Ok(path)
    }
}

fn to_verbosity(value: u64) -> log::LevelFilter {
    match value {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    }
}
