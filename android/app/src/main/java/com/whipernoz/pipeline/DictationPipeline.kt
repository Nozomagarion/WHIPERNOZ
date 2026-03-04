package com.whipernoz.pipeline

import android.util.Log
import com.whipernoz.whisper.WhisperEngine
import com.whipernoz.llm.LlmProcessor

/**
 * Central dictation pipeline for Android:
 * Audio → Whisper STT → Regex cleanup → LLM cleanup → Dictionary → Inject text
 */
class DictationPipeline(
    private val whisperEngine: WhisperEngine,
    private val llmProcessor: LlmProcessor,
) {
    companion object {
        private const val TAG = "DictationPipeline"

        // Common filler words to remove (English + French)
        private val FILLER_REGEX = Regex(
            "\\b(um|uh|er|erm|hmm|like|you know|basically|sort of|kind of|I mean|euh|heu|bah|ben|genre|en fait|du coup|voilà)\\b",
            RegexOption.IGNORE_CASE
        )
        private val MULTI_SPACE_REGEX = Regex("\\s{2,}")
    }

    /**
     * Process audio samples through the full pipeline
     */
    suspend fun process(audioSamples: FloatArray, language: String = "auto"): String {
        // Step 1: Whisper transcription
        Log.d(TAG, "Step 1: Transcribing ${audioSamples.size} samples...")
        val rawTranscript = whisperEngine.transcribe(audioSamples, language)
        Log.d(TAG, "Raw transcript: $rawTranscript")

        if (rawTranscript.isBlank()) return ""

        // Step 2: Regex pre-cleanup
        val regexCleaned = regexCleanup(rawTranscript)
        Log.d(TAG, "Regex cleaned: $regexCleaned")

        // Step 3: LLM cleanup (punctuation, grammar, tone)
        val llmCleaned = llmProcessor.cleanup(regexCleaned)
        Log.d(TAG, "LLM cleaned: $llmCleaned")

        // Step 4: Personal dictionary corrections (TODO)

        return llmCleaned
    }

    /**
     * Remove filler words and normalize whitespace
     */
    private fun regexCleanup(text: String): String {
        var cleaned = FILLER_REGEX.replace(text, "")
        cleaned = MULTI_SPACE_REGEX.replace(cleaned, " ").trim()

        // Capitalize first letter
        return cleaned.replaceFirstChar { it.uppercase() }
    }
}
