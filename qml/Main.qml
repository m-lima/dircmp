import QtQuick as Q

Q.Item {
  DirectoryInput {
    anchors {
      top: parent.top
    }
  }

  Navigation {
    onNext: showBack(true)
    onBack: showBack(false)

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
