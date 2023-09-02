import QtQuick as Q
import QtQuick.Controls as C

Q.Item {
  SlidingContainer {
    id: slidingContainer

    // TODO: Make my own drop location
    C.TextField {
      anchors {
        top: parent.top
      }
    }

    C.TextField {
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
