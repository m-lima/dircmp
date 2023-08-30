pub fn run() -> std::process::ExitCode {
    use ::slint::ComponentHandle;

    let ui = slint::MainWindow::new().unwrap();
    let weak = ui.as_weak();
    ui.on_yo(move || {
        let ui = weak.upgrade().unwrap();
        ui.set_bla(ui.get_bla() + 1);
    });

    ui.run().unwrap();
    0.into()
}

mod slint {
    #![allow(clippy::float_cmp)]

    slint::slint! {
        import { MainWindow } from "slint/ui.slint";
    }
}
