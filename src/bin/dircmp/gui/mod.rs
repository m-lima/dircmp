qmetaobject::qrc!(qml, "qml" as "/" {
    "qtquickcontrols2.conf",
    "App.qml",
    "Main.qml",
    "Navigation.qml",
    "navigation/Button.qml",
    "DirectoryInput.qml",
});

qmetaobject::qrc!(img, "res/img" as "/img" {
    "left.svg",
});

pub fn run() -> std::process::ExitCode {
    qml();
    img();

    let mut engine = qmetaobject::QmlEngine::new();
    engine.load_file(qmetaobject::QString::from("qrc:/App.qml"));
    engine.exec();

    std::process::ExitCode::SUCCESS
}
