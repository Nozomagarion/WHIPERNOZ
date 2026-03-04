package com.whipernoz.service

import android.accessibilityservice.AccessibilityService
import android.os.Bundle
import android.view.accessibility.AccessibilityEvent
import android.view.accessibility.AccessibilityNodeInfo
import android.util.Log

/**
 * Accessibility Service that detects when a text field is focused
 * and shows/hides the floating microphone button.
 *
 * This is the core of the Android experience:
 * When a keyboard appears (text field focused), the floating mic button appears.
 * User taps the mic, speaks, and text is injected into the focused field.
 */
class WhipernozAccessibilityService : AccessibilityService() {

    companion object {
        private const val TAG = "WhipernozA11y"

        // Text field class names to detect
        private val TEXT_FIELD_CLASSES = setOf(
            "android.widget.EditText",
            "android.widget.AutoCompleteTextView",
            "android.widget.MultiAutoCompleteTextView",
            "android.widget.TextView", // contentEditable in WebViews
        )
    }

    override fun onAccessibilityEvent(event: AccessibilityEvent?) {
        event ?: return

        when (event.eventType) {
            AccessibilityEvent.TYPE_VIEW_FOCUSED -> handleFocusChange(event)
            AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED -> handleWindowChange(event)
            else -> {}
        }
    }

    private fun handleFocusChange(event: AccessibilityEvent) {
        val source = event.source ?: return

        if (isTextField(source)) {
            Log.d(TAG, "Text field focused: ${source.className} in ${event.packageName}")
            showFloatingButton(source)
        } else {
            hideFloatingButton()
        }

        source.recycle()
    }

    private fun handleWindowChange(event: AccessibilityEvent) {
        // Check if keyboard appeared (soft input visible)
        // This is a secondary signal for text field detection
        Log.d(TAG, "Window state changed: ${event.packageName}")
    }

    private fun isTextField(node: AccessibilityNodeInfo): Boolean {
        val className = node.className?.toString() ?: return false

        // Check if it's a known text field class
        if (className in TEXT_FIELD_CLASSES && node.isEditable) {
            return true
        }

        // Check for contentEditable WebView fields
        if (node.isEditable && node.isFocusable) {
            return true
        }

        return false
    }

    private fun showFloatingButton(textField: AccessibilityNodeInfo) {
        // Get the bounding rectangle of the text field
        val rect = android.graphics.Rect()
        textField.getBoundsInScreen(rect)

        Log.d(TAG, "Showing floating button near: $rect")

        // TODO: Start FloatingButtonService and position near the text field
        // FloatingButtonService.show(this, rect.right, rect.top)
    }

    private fun hideFloatingButton() {
        // TODO: Hide FloatingButtonService
        // FloatingButtonService.hide(this)
    }

    /**
     * Inject text into the currently focused text field
     */
    fun injectText(text: String) {
        val focusedNode = rootInActiveWindow?.findFocus(AccessibilityNodeInfo.FOCUS_INPUT)
        if (focusedNode != null && focusedNode.isEditable) {
            val args = Bundle().apply {
                putCharSequence(
                    AccessibilityNodeInfo.ACTION_ARGUMENT_SET_TEXT_CHARSEQUENCE,
                    text
                )
            }
            focusedNode.performAction(AccessibilityNodeInfo.ACTION_SET_TEXT, args)
            Log.d(TAG, "Text injected: ${text.take(50)}...")
            focusedNode.recycle()
        } else {
            Log.w(TAG, "No editable focused field found for text injection")
        }
    }

    override fun onInterrupt() {
        Log.d(TAG, "Service interrupted")
    }

    override fun onServiceConnected() {
        super.onServiceConnected()
        Log.d(TAG, "Accessibility service connected")
    }
}
