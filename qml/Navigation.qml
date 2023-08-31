import QtQuick as Q

import "navigation"

Q.Item {
  id: root

  signal back()
  signal next()

  function showBack(bool) { state = bool ? 'withBack' : 'noBack' }

  implicitHeight: 50
  state: 'noBack'

  Button {
    id: back

    text: 'Back'

    anchors {
      top: parent.top
      bottom: parent.bottom
      left: parent.left
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

  states: [
    Q.State {
      name: 'noBack'

      Q.PropertyChanges {
        target: back
        width: 0
      }
    },
    Q.State {
      name: 'withBack'

      Q.PropertyChanges {
        target: back
        width: root.width / 2
      }
    }
  ]

  transitions: [
    Q.Transition {
      Q.NumberAnimation {
        duration: 200
        property: 'width'
      }
    }
  ]
}
