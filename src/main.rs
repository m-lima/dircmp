mod args;
mod cli;
mod error;
mod gui;
mod hasher;
mod thread;

fn main() -> std::process::ExitCode {
    let args = args::parse();
    if let Some(args::Cli::Cli(args)) = args.cli {
        cli::run(args)
    } else {
        gui::run()
    }
}
