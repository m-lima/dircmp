# Development

## QML location for `qmlls`
> qmlls could not find a build directory, without a build directory containing a current build there could be spurious warnings, you might want to pass the --build-dir <buildDir> option to qmlls, or set the environment variable QMLLS_BUILD_DIRS

If QT is not globally installed, and the `INSTALL_PREFIX` for QT does not point to its current location, `qmlls` will not work. The value to set the env var is:
```bash
$ QMLLS_BUILD_DIRS=<PATH_TO_QT>/share/qt/qml
```

# Build

## QT location
> Failed to execute qmake. Make sure 'qmake' is in your path!
> Cannot open `...`, please make sure that the Qt headers are installed.

### Dynamic
If QT is installed globally (and dynamically), you are good to go.

If QT is not globally installed, `QMAKE` needs to be set:
```bash
$ export QMAKE=<PATH_TO_QT>/bin/qmake
```

This may fail if `qmake -query` points to invalid paths. In this case, two more variables need to be set:
```bash
$ export QT_INCLUDE_PATH=<PATH_TO_QT>/include
$ export QT_LIBRARY_PATH=<PATH_TO_QT>/lib
```

### Static

**TODO**

# Runtime

## Dynamic

### Platform missing
> Could not find the Qt platform plugin "`...`" in "`...`"

If the runtime cannot load the QML platform plugins, the variable `QT_QPA_PLATFORM_PLUGIN_PATH` needs to be set:
```bash
$ export QT_QPA_PLATFORM_PLUGIN_PATH=<PATH_TO_QT>/share/qt/plugins/platforms
```

### Modules missing
> QQmlApplicationEngine failed to load component

If the runtime cannot load QML modules, the variable `QML2_IMPORT_PATH` needs to be set:
```bash
$ export QML2_IMPORT_PATH=<QT_DIR>/share/qt/qml
```
