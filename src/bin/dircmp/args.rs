#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Path does not exist")]
    PathDoesNotExist,
    #[error("Path is not a directory")]
    PathNotDir,
}

pub fn parse() -> Option<Args> {
    <App as clap::Parser>::parse().cli.map(Command::args)
}

#[derive(Debug, clap::Parser)]
struct App {
    #[command(subcommand)]
    pub cli: Option<Command>,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    Cli(Args),
}

impl Command {
    fn args(self) -> Args {
        match self {
            Command::Cli(args) => args,
        }
    }
}

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Verbosity level
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbosity: u8,
    /// Show matched items
    #[arg(short, long)]
    pub matched: bool,
    /// Path to the `left` directory to compare
    #[arg(value_parser = clap::builder::TypedValueParser::try_map(clap::builder::OsStringValueParser::new(), parse_dir))]
    pub left: std::path::PathBuf,
    /// Path to the `right` directory to compare
    #[arg(value_parser = clap::builder::TypedValueParser::try_map(clap::builder::OsStringValueParser::new(), parse_dir))]
    pub right: std::path::PathBuf,
}

impl Args {
    pub fn verbosity(&self) -> log::LevelFilter {
        to_verbosity(self.verbosity)
    }
}

fn parse_dir(input: std::ffi::OsString) -> Result<std::path::PathBuf, Error> {
    let path = std::path::PathBuf::from(input);

    if !path.exists() {
        Err(Error::PathDoesNotExist)
    } else if !path.is_dir() {
        Err(Error::PathNotDir)
    } else {
        Ok(path)
    }
}

fn to_verbosity(value: u8) -> log::LevelFilter {
    match value {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    }
}
