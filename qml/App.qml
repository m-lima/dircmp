import QtQuick.Controls.Basic as C

C.ApplicationWindow {
  id: root

  title: 'DirCmp'
  visible: true
  width: 600
  height: 600

  palette {
    window: '#353535'
    base: '#2a2a2a'

    mid: '#808080'

    highlight: '#006000'
    highlightedText: '#cccccc'

    text: '#cccccc'
    placeholderText: '#999999'
    windowText: '#cccccc'
  }

  // Button {
  //   text: 'Yo'
  // }

  DirectoryInput {
    anchors {
      top: parent.top
    }
  }
  BigButton {
    id: next

    color: 'green'
    palette.buttonText: '#252525'

    height: parent.height

    text: 'Big guy'
  }
  DirectoryInput {
    x: 200
  }
}
