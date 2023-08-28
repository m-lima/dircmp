mod args;
mod cli;
mod gui;

fn main() -> std::process::ExitCode {
    if let Some(args) = args::parse() {
        cli::run(args)
    } else {
        gui::run()
    }
}
