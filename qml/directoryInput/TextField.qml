import QtQuick as Q
import QtQuick.Controls as C
import QtQuick.Templates as T

T.TextField {
  id: root

    implicitWidth: implicitBackgroundWidth + leftInset + rightInset
      || Math.max(contentWidth, placeholder.implicitWidth) + leftPadding + rightPadding
    implicitHeight: Math.max(
      implicitBackgroundHeight + topInset + bottomInset,
      contentHeight + topPadding + bottomPadding,
      placeholder.implicitHeight + topPadding + bottomPadding
    )

    padding: 6
    leftPadding: padding + 4

    color: root.palette.text
    selectionColor: root.palette.highlight
    selectedTextColor: root.palette.highlightedText
    placeholderTextColor: root.palette.placeholderText
    verticalAlignment: Q.TextInput.AlignVCenter

    C.PlaceholderText {
        id: placeholder

        x: root.leftPadding
        y: root.topPadding
        width: root.width - (root.leftPadding + root.rightPadding)
        height: root.height - (root.topPadding + root.bottomPadding)

        text: root.placeholderText
        font: root.font
        color: root.placeholderTextColor
        verticalAlignment: root.verticalAlignment
        visible: !root.length && !root.preeditText && (!root.activeFocus || root.horizontalAlignment !== Qt.AlignHCenter)
        elide: Q.Text.ElideRight
        renderType: root.renderType
    }

    background: Q.Rectangle {
        implicitWidth: 200
        implicitHeight: 40
        border.width: root.activeFocus ? 2 : 1
        color: root.palette.base
        border.color: root.activeFocus ? root.palette.highlight : root.palette.mid
    }

}
