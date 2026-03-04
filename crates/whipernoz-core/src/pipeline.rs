use anyhow::Result;

/// Events emitted by the dictation pipeline to the UI
#[derive(Debug, Clone)]
pub enum PipelineEvent {
    RecordingStarted,
    AudioLevel(f32),
    SpeechDetected,
    SilenceDetected,
    TranscribingStarted,
    RawTranscript(String),
    CleaningStarted,
    CleanTranscript(String),
    TextReady(String),
    Error(String),
}

/// Central orchestrator: Audio → Whisper → Cleanup → LLM → Dictionary → Final text
pub struct DictationPipeline {
    // Will hold: WhisperEngine, LlmEngine, PersonalDictionary, ShortcutEngine, PipelineConfig
}

impl DictationPipeline {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
