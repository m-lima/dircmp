#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

pub fn run() -> std::process::ExitCode {
    let ui = MainWindow::new().unwrap();

    ui.on_result_path_changed(|path| {
        println!("{path}");
    });

    let ptr = ui.as_weak();
    ui.on_load(move || {
        println!(
            "Out {}",
            ptr.upgrade().map_or_else(
                || {
                    let mut string = slint::SharedString::new();
                    string.push_str("Failed upgrade");
                    string
                    // String::from("Failed upgrade")
                },
                |ui| ui.get_result_path()
            )
        );
        let ui = ptr.upgrade().unwrap();
        let ptr = ui.as_weak();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(5));
            println!(
                "In {}",
                ptr.upgrade().map_or_else(
                    || {
                        let mut string = slint::SharedString::new();
                        string.push_str("Failed upgrade");
                        string
                        // String::from("Failed upgrade")
                    },
                    |ui| ui.get_result_path()
                )
            );
        });
    });

    ui.run().unwrap();
    0.into()
}
