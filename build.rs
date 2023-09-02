fn main() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("slint")
        .join("ui.slint");
    slint_build::compile_with_config(
        path,
        slint_build::CompilerConfiguration::new().with_style(String::from("fluent")),
    )
    .unwrap();
}
