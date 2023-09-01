import QtQuick as Q

Q.Item {
  SlidingContainer {
    id: slidingContainer

    DirectoryInput {
      anchors {
        top: parent.top
      }
    }

    DirectoryInput {
      x: 200
    }
  }

  Navigation {
    onNext: {
      slidingContainer.next()
      showBack(true)
    }

    onBack: {
      slidingContainer.back()
      showBack(slidingContainer.index === 0)
    }

    anchors {
      bottom: parent.bottom
      right: parent.right
      left: parent.left
    }
  }
}
