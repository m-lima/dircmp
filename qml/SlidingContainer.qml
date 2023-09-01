import QtQuick as Q

Q.Item {
  id: root

  readonly property int index: 0

  function next() {
    index = Math.min(children.length - 1, index + 1)
  }

  function back() {
    index = Math.max(0, index - 1)
  }
}
