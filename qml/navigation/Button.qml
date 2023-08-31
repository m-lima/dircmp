import QtQuick as Q
import QtQuick.Templates as T

T.Button {
  id: root

  property Q.color color: palette.button
  property Q.color textColor: palette.buttonText

  implicitWidth: Math.max(
    implicitBackgroundWidth + leftInset + rightInset,
    implicitContentWidth + leftPadding + rightPadding
  )
  implicitHeight: Math.max(
    implicitBackgroundHeight + topInset + bottomInset,
    implicitContentHeight + topPadding + bottomPadding
  )

  padding: 6
  spacing: 6
  horizontalPadding: padding + 2

  font {
    pointSize: 18
    bold: true
  }

  contentItem: Q.Text {
    horizontalAlignment: Q.Text.AlignHCenter
    verticalAlignment: Q.Text.AlignVCenter
    text: root.text
    font: root.font
    color: root.textColor
  }

  background: Q.Rectangle {
    id: background

    color: root.down
      ? root.color.darker(1.1)
      : root.hovered
        ? root.color.lighter(1.1)
        : root.color

    gradient: gradient

    Q.Gradient {
      id: gradient

      Q.GradientStop {
        position: 0
        color: background.color.lighter(1.1)
      }

      Q.GradientStop {
        position: 1
        color: background.color.darker(1.1)
      }
    }

    Q.HoverHandler {
      cursorShape: Qt.PointingHandCursor
    }
  }
}
