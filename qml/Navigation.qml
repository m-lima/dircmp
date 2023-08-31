import QtQuick as Q

import "navigation"

Q.Item {
  id: root

  property bool showBack: false

  signal back()
  signal next()

  implicitHeight: 50

  Button {
    id: back

    text: 'Back'
    // color: '#803030'
    // textColor: '#252525'
    // TODO: This expression coupled with the animation, makes it wonky to resize the window
    width: root.showBack ? parent.width / 2 : 0

    anchors {
      top: parent.top
      bottom: parent.bottom
      left: parent.left
    }

    Q.Behavior on width {
      Q.NumberAnimation {
        duration: 200
      }
    }

    onClicked: root.back()
  }

  Button {
    id: next

    color: '#008000'
    textColor: '#252525'
    text: 'Next'

    anchors {
      top: parent.top
      bottom: parent.bottom
      right: parent.right
      left: back.right
    }

    onClicked: root.next()
  }
}
