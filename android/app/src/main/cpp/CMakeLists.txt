cmake_minimum_required(VERSION 3.22.1)
project("whisper_jni")

# TODO: Add whisper.cpp source files when integrating
# For now, create a stub JNI library

add_library(whisper_jni SHARED
    whisper_jni.cpp
)

target_link_libraries(whisper_jni
    android
    log
)
