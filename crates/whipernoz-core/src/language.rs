use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub code: String,
    pub name: String,
    pub whisper_code: Option<String>,
}

/// Supported languages (subset - Whisper supports ~100)
pub fn supported_languages() -> Vec<LanguageConfig> {
    vec![
        LanguageConfig { code: "en".into(), name: "English".into(), whisper_code: Some("en".into()) },
        LanguageConfig { code: "fr".into(), name: "Français".into(), whisper_code: Some("fr".into()) },
        LanguageConfig { code: "es".into(), name: "Español".into(), whisper_code: Some("es".into()) },
        LanguageConfig { code: "de".into(), name: "Deutsch".into(), whisper_code: Some("de".into()) },
        LanguageConfig { code: "it".into(), name: "Italiano".into(), whisper_code: Some("it".into()) },
        LanguageConfig { code: "pt".into(), name: "Português".into(), whisper_code: Some("pt".into()) },
        LanguageConfig { code: "ja".into(), name: "日本語".into(), whisper_code: Some("ja".into()) },
        LanguageConfig { code: "zh".into(), name: "中文".into(), whisper_code: Some("zh".into()) },
        LanguageConfig { code: "ko".into(), name: "한국어".into(), whisper_code: Some("ko".into()) },
        LanguageConfig { code: "ar".into(), name: "العربية".into(), whisper_code: Some("ar".into()) },
        LanguageConfig { code: "auto".into(), name: "Auto-detect".into(), whisper_code: None },
    ]
}
