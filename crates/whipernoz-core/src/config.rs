use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub whisper: WhisperConfig,
    pub llm: LlmConfig,
    pub hotkey: String,
    pub language: String,
    pub tone_mode: ToneMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperConfig {
    pub model_path: String,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub mode: LlmMode,
    pub local_model_path: Option<String>,
    pub api_key: Option<String>,
    pub api_provider: Option<ApiProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LlmMode {
    Local,
    Cloud,
    RegexOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiProvider {
    Claude,
    OpenAI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToneMode {
    Auto,
    Professional,
    Casual,
    Technical,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            whisper: WhisperConfig {
                model_path: "models/whisper/ggml-base.en.bin".to_string(),
                language: Some("en".to_string()),
            },
            llm: LlmConfig {
                mode: LlmMode::RegexOnly,
                local_model_path: None,
                api_key: None,
                api_provider: None,
            },
            hotkey: "Ctrl+Shift+Space".to_string(),
            language: "en".to_string(),
            tone_mode: ToneMode::Auto,
        }
    }
}
