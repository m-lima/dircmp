qmetaobject::qrc!(qml, "qml" as "/" {
    "qtquickcontrols2.conf",
    "App.qml",
    "BigButton.qml",
    "DirectoryInput.qml",
});

pub fn run() -> std::process::ExitCode {
    qml();

    let mut engine = qmetaobject::QmlEngine::new();
    engine.load_file(qmetaobject::QString::from("qrc:/App.qml"));
    engine.exec();

    std::process::ExitCode::SUCCESS
}
