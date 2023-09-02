#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

pub fn run() -> std::process::ExitCode {
    let ui = MainWindow::new().unwrap();

    ui.on_result_path_changed(|path| {
        println!("{path}");
    });

    ui.run().unwrap();
    0.into()
}
