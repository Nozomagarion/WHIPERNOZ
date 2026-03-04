use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Voice shortcuts: trigger phrases that expand to full text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutEngine {
    shortcuts: HashMap<String, String>,
}

impl ShortcutEngine {
    pub fn new() -> Self {
        Self {
            shortcuts: HashMap::new(),
        }
    }

    pub fn add_shortcut(&mut self, trigger: String, expansion: String) {
        self.shortcuts.insert(trigger.to_lowercase(), expansion);
    }

    pub fn remove_shortcut(&mut self, trigger: &str) {
        self.shortcuts.remove(&trigger.to_lowercase());
    }

    /// Check if the transcript matches a shortcut trigger, return the expansion
    pub fn try_expand(&self, transcript: &str) -> Option<String> {
        let lower = transcript.to_lowercase().trim().to_string();
        self.shortcuts.get(&lower).cloned()
    }
}

impl Default for ShortcutEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_matching_shortcut() {
        let mut engine = ShortcutEngine::new();
        engine.add_shortcut(
            "insert signature".to_string(),
            "Best regards,\nJohn Doe\nSoftware Engineer".to_string(),
        );

        let result = engine.try_expand("insert signature");
        assert_eq!(
            result,
            Some("Best regards,\nJohn Doe\nSoftware Engineer".to_string())
        );
    }

    #[test]
    fn returns_none_for_no_match() {
        let engine = ShortcutEngine::new();
        assert_eq!(engine.try_expand("hello world"), None);
    }
}
