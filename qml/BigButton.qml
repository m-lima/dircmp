import QtQuick as Q
import QtQuick.Templates as T

T.Button {
  id: control

  property Q.color color: palette.button
  property Q.color textColor: palette.buttonText

  implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset,
                          implicitContentWidth + leftPadding + rightPadding)
  implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset,
                           implicitContentHeight + topPadding + bottomPadding)

  padding: 6
  spacing: 6
  horizontalPadding: padding + 2

  font {
    pointSize: 18
    bold: true
  }

  contentItem: Q.Text {
    horizontalAlignment: Q.Text.AlignCenter
    verticalAlignment: Q.Text.AlignVCenter
    text: control.text
    font: control.font
    color: control.textColor
  }

  background: Q.Rectangle {
    id: background

    color: control.down
      ? control.color.darker(1.1)
      : control.hovered
        ? control.color.lighter(1.1)
        : control.color

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
