package com.whipernoz.llm

import android.util.Log
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import retrofit2.http.Body
import retrofit2.http.Header
import retrofit2.http.POST

/**
 * LLM processor for text cleanup on Android.
 * Uses Claude or OpenAI API since local LLM is too heavy for most phones.
 */
class LlmProcessor {

    companion object {
        private const val TAG = "LlmProcessor"
    }

    private var apiKey: String? = null
    private var provider: ApiProvider = ApiProvider.CLAUDE

    enum class ApiProvider { CLAUDE, OPENAI }

    fun configure(apiKey: String, provider: ApiProvider) {
        this.apiKey = apiKey
        this.provider = provider
    }

    /**
     * Clean up raw transcript using LLM API
     */
    suspend fun cleanup(rawText: String, context: String = ""): String = withContext(Dispatchers.IO) {
        val key = apiKey
        if (key == null) {
            Log.w(TAG, "No API key configured, returning raw text")
            return@withContext rawText
        }

        try {
            when (provider) {
                ApiProvider.CLAUDE -> cleanupWithClaude(rawText, context, key)
                ApiProvider.OPENAI -> cleanupWithOpenAI(rawText, context, key)
            }
        } catch (e: Exception) {
            Log.e(TAG, "LLM cleanup failed, returning raw text", e)
            rawText
        }
    }

    private suspend fun cleanupWithClaude(text: String, context: String, apiKey: String): String {
        // TODO: Implement Claude API call
        Log.d(TAG, "Claude cleanup called for: ${text.take(50)}")
        return text // Placeholder
    }

    private suspend fun cleanupWithOpenAI(text: String, context: String, apiKey: String): String {
        // TODO: Implement OpenAI API call
        Log.d(TAG, "OpenAI cleanup called for: ${text.take(50)}")
        return text // Placeholder
    }
}
