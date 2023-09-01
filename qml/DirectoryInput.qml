import QtQuick as Q
import Qt.labs.platform as P

import "directoryInput"

Q.DropArea {
  id: root

  function toURL(url) {
    return new URL(url);
  }

  function isDir(url) {
    return url.protocol === 'file:' && url.pathname.endsWith('/');
  }

  function set(url) {
    textField.text = toURL(url[0]).pathname;
    return true;
  }

  implicitWidth: textField.width + button.width
  implicitHeight: textField.height

  height: parent.height
  keys: ['text/uri-list']
  onEntered: (evt) => evt.accepted = evt.urls.map(toURL).filter(isDir).length === 1
  onDropped: (evt) => set(evt.urls) && evt.accept()

  TextField {
    id: textField

    placeholderText: 'Path to directory to compare'
    validator: Q.RegularExpressionValidator {
      regularExpression: /.+\//
    }
    palette.base: !text || acceptableInput ? root.palette.base : '#352020'

    onPressed: folderDialog.open()
  }

  Button {
    id: button
  }

  P.FolderDialog {
    id: folderDialog

    options: P.FolderDialog.ShowDirsOnly | P.FolderDialog.ReadOnly
    folder: textField.text
      ? textField.text
      : P.StandardPaths.standardLocations(P.StandardPaths.HomeLocation)[0]

    onAccepted: textField.text = folderDialog.folder
  }
}
