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

    button: '#444444'
    buttonText: '#aaaaaa'

    highlight: '#006000'
    highlightedText: '#cccccc'

    text: '#cccccc'
    placeholderText: '#999999'
    windowText: '#cccccc'
  }

  DirectoryInput {
    anchors {
      top: parent.top
    }
  }

  Navigation {
    onNext: showBack = true
    onBack: showBack = false

    anchors {
      bottom: parent.bottom
      right: parent.right
      left: parent.left
    }
  }

  DirectoryInput {
    x: 200
  }
}
