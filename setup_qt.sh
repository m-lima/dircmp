if [ ${1} ]
then
  export QT_DIR="${1}"
  export QMLLS_BUILD_DIRS="${1}"/share/qt/qml
  export QMAKE="${1}"/bin/qmake
  export QT_INCLUDE_PATH="${1}"/include
  export QT_LIBRARY_PATH="${1}"/lib
  export QT_QPA_PLATFORM_PLUGIN_PATH="${1}"/share/qt/plugins/platforms
  export QML2_IMPORT_PATH="${1}"/share/qt/qml
else
  echo '[31mExpected the root installation for QT[m' >&2
fi
