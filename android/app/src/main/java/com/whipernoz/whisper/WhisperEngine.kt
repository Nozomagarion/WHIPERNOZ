package com.whipernoz.whisper

import android.content.Context
import android.util.Log
import java.io.File

/**
 * Whisper STT engine wrapping whisper.cpp via JNI.
 * Loads and runs the Whisper model for local speech-to-text.
 */
class WhisperEngine(private val context: Context) {

    companion object {
        private const val TAG = "WhisperEngine"

        init {
            System.loadLibrary("whisper_jni")
        }
    }

    private var modelPointer: Long = 0

    /**
     * Initialize the Whisper model from the given path
     */
    fun initialize(modelPath: String): Boolean {
        val file = File(modelPath)
        if (!file.exists()) {
            Log.e(TAG, "Model file not found: $modelPath")
            return false
        }

        modelPointer = nativeInit(modelPath)
        val success = modelPointer != 0L
        if (success) {
            Log.d(TAG, "Whisper model loaded from: $modelPath")
        } else {
            Log.e(TAG, "Failed to load Whisper model")
        }
        return success
    }

    /**
     * Transcribe audio samples (16kHz mono float32)
     */
    fun transcribe(samples: FloatArray, language: String = "auto"): String {
        if (modelPointer == 0L) {
            Log.e(TAG, "Model not initialized")
            return ""
        }

        val startTime = System.nanoTime()
        val result = nativeTranscribe(modelPointer, samples, language)
        val elapsed = (System.nanoTime() - startTime) / 1_000_000
        Log.d(TAG, "Transcription completed in ${elapsed}ms: ${result.take(100)}")

        return result
    }

    /**
     * Release the model resources
     */
    fun release() {
        if (modelPointer != 0L) {
            nativeFree(modelPointer)
            modelPointer = 0
            Log.d(TAG, "Whisper model released")
        }
    }

    // JNI native methods (implemented in whisper_jni.cpp)
    private external fun nativeInit(modelPath: String): Long
    private external fun nativeTranscribe(modelPtr: Long, samples: FloatArray, language: String): String
    private external fun nativeFree(modelPtr: Long)
}
