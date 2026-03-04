use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RecordingStatus {
    pub is_recording: bool,
    pub state: String,
}

#[tauri::command]
pub async fn start_recording() -> Result<String, String> {
    tracing::info!("Recording started");
    // TODO: Start audio capture pipeline
    // For now, return a placeholder
    Ok("Recording started - Whisper integration coming soon!".to_string())
}

#[tauri::command]
pub async fn stop_recording() -> Result<String, String> {
    tracing::info!("Recording stopped");
    // TODO: Stop capture, run Whisper, cleanup with LLM
    Ok("Recording stopped - transcription coming soon!".to_string())
}

#[tauri::command]
pub fn get_status() -> RecordingStatus {
    RecordingStatus {
        is_recording: false,
        state: "idle".to_string(),
    }
}
