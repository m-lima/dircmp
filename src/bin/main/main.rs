mod args;
mod cli;
mod gui;

fn main() -> std::process::ExitCode {
    let args = args::parse();
    if let Some(args::Command::Cli(args)) = args.cli {
        cli::run(args)
    } else {
        gui::run()
    }
}
