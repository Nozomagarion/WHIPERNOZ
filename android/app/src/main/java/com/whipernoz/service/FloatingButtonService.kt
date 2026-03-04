package com.whipernoz.service

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Intent
import android.graphics.PixelFormat
import android.os.Build
import android.os.IBinder
import android.util.Log
import android.view.Gravity
import android.view.MotionEvent
import android.view.View
import android.view.WindowManager
import android.widget.ImageView
import androidx.core.app.NotificationCompat

/**
 * Floating microphone button that appears when a text field is focused.
 * The user taps it to start/stop voice dictation.
 *
 * Uses TYPE_ACCESSIBILITY_OVERLAY (no SYSTEM_ALERT_WINDOW permission needed
 * when launched from an AccessibilityService).
 */
class FloatingButtonService : Service() {

    companion object {
        private const val TAG = "FloatingButton"
        private const val CHANNEL_ID = "whipernoz_floating"
        private const val NOTIFICATION_ID = 1
    }

    private var windowManager: WindowManager? = null
    private var floatingView: View? = null
    private var isRecording = false

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onCreate() {
        super.onCreate()
        createNotificationChannel()
        startForeground(NOTIFICATION_ID, createNotification())
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        val x = intent?.getIntExtra("x", 0) ?: 0
        val y = intent?.getIntExtra("y", 0) ?: 0
        showButton(x, y)
        return START_STICKY
    }

    private fun showButton(x: Int, y: Int) {
        if (floatingView != null) {
            updatePosition(x, y)
            return
        }

        windowManager = getSystemService(WINDOW_SERVICE) as WindowManager

        // Create the floating mic button
        val button = ImageView(this).apply {
            // TODO: Set mic icon drawable
            setBackgroundColor(0xFF667EEA.toInt())
            setPadding(16, 16, 16, 16)
        }

        val params = WindowManager.LayoutParams(
            120, 120,
            WindowManager.LayoutParams.TYPE_ACCESSIBILITY_OVERLAY,
            WindowManager.LayoutParams.FLAG_NOT_FOCUSABLE or
                    WindowManager.LayoutParams.FLAG_LAYOUT_NO_LIMITS,
            PixelFormat.TRANSLUCENT
        ).apply {
            gravity = Gravity.TOP or Gravity.START
            this.x = x
            this.y = y
        }

        // Make draggable + clickable
        var initialX = 0
        var initialY = 0
        var initialTouchX = 0f
        var initialTouchY = 0f

        button.setOnTouchListener { _, event ->
            when (event.action) {
                MotionEvent.ACTION_DOWN -> {
                    initialX = params.x
                    initialY = params.y
                    initialTouchX = event.rawX
                    initialTouchY = event.rawY
                    true
                }
                MotionEvent.ACTION_MOVE -> {
                    params.x = initialX + (event.rawX - initialTouchX).toInt()
                    params.y = initialY + (event.rawY - initialTouchY).toInt()
                    windowManager?.updateViewLayout(button, params)
                    true
                }
                MotionEvent.ACTION_UP -> {
                    val dx = event.rawX - initialTouchX
                    val dy = event.rawY - initialTouchY
                    // If barely moved, treat as click
                    if (Math.abs(dx) < 10 && Math.abs(dy) < 10) {
                        toggleRecording()
                    }
                    true
                }
                else -> false
            }
        }

        windowManager?.addView(button, params)
        floatingView = button
        Log.d(TAG, "Floating button shown at ($x, $y)")
    }

    private fun updatePosition(x: Int, y: Int) {
        val view = floatingView ?: return
        val params = view.layoutParams as WindowManager.LayoutParams
        params.x = x
        params.y = y
        windowManager?.updateViewLayout(view, params)
    }

    private fun toggleRecording() {
        isRecording = !isRecording
        if (isRecording) {
            Log.d(TAG, "Recording started")
            // TODO: Start audio capture, change button color to red
        } else {
            Log.d(TAG, "Recording stopped")
            // TODO: Stop audio capture, process with Whisper + LLM, inject text
        }
    }

    fun hide() {
        floatingView?.let {
            windowManager?.removeView(it)
            floatingView = null
        }
    }

    override fun onDestroy() {
        hide()
        super.onDestroy()
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                "Whipernoz Dictation",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "Voice dictation service"
            }
            val manager = getSystemService(NotificationManager::class.java)
            manager.createNotificationChannel(channel)
        }
    }

    private fun createNotification(): Notification {
        return NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle("Whipernoz")
            .setContentText("Voice dictation active")
            .setSmallIcon(android.R.drawable.ic_btn_speak_now)
            .setPriority(NotificationCompat.PRIORITY_LOW)
            .build()
    }
}
