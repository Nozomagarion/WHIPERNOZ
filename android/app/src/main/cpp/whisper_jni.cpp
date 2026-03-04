#include <jni.h>
#include <string>
#include <android/log.h>

#define TAG "WhisperJNI"
#define LOGD(...) __android_log_print(ANDROID_LOG_DEBUG, TAG, __VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, TAG, __VA_ARGS__)

// TODO: Include whisper.cpp headers when integrating
// #include "whisper.h"

extern "C" {

JNIEXPORT jlong JNICALL
Java_com_whipernoz_whisper_WhisperEngine_nativeInit(
    JNIEnv *env, jobject thiz, jstring model_path) {

    const char *path = env->GetStringUTFChars(model_path, nullptr);
    LOGD("Initializing Whisper model from: %s", path);

    // TODO: Load whisper model
    // struct whisper_context *ctx = whisper_init_from_file(path);

    env->ReleaseStringUTFChars(model_path, path);

    // Return stub pointer for now
    return 0;
}

JNIEXPORT jstring JNICALL
Java_com_whipernoz_whisper_WhisperEngine_nativeTranscribe(
    JNIEnv *env, jobject thiz, jlong model_ptr, jfloatArray samples, jstring language) {

    // TODO: Run whisper inference
    LOGD("Transcribe called (stub)");

    return env->NewStringUTF("[Whisper transcription placeholder]");
}

JNIEXPORT void JNICALL
Java_com_whipernoz_whisper_WhisperEngine_nativeFree(
    JNIEnv *env, jobject thiz, jlong model_ptr) {

    // TODO: Free whisper context
    // whisper_free((struct whisper_context *)model_ptr);
    LOGD("Whisper model freed");
}

} // extern "C"
