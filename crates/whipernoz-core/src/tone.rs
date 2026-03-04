use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tone {
    Professional,
    Casual,
    Technical,
    Neutral,
}

/// Detect appropriate tone based on the active application context
pub fn detect_tone(app_name: &str, window_title: &str) -> Tone {
    let app = app_name.to_lowercase();
    let title = window_title.to_lowercase();

    // Email clients → professional
    if app.contains("outlook") || app.contains("thunderbird")
        || title.contains("gmail") || title.contains("mail")
    {
        return Tone::Professional;
    }

    // Chat apps → casual
    if app.contains("slack") || app.contains("discord") || app.contains("whatsapp")
        || app.contains("telegram") || app.contains("messenger")
    {
        return Tone::Casual;
    }

    // Code editors → technical
    if app.contains("code") || app.contains("intellij") || app.contains("vim")
        || app.contains("neovim") || app.contains("android studio")
    {
        return Tone::Technical;
    }

    Tone::Neutral
}

impl Tone {
    /// Get the LLM prompt modifier for this tone
    pub fn prompt_instruction(&self) -> &str {
        match self {
            Tone::Professional => "Use a professional, formal tone. Proper grammar and complete sentences.",
            Tone::Casual => "Use a casual, friendly tone. Contractions and informal language are fine.",
            Tone::Technical => "Use precise technical language. Preserve code-related terms exactly.",
            Tone::Neutral => "Use clear, neutral language with proper punctuation.",
        }
    }
}
