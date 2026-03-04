use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Personal dictionary for custom term corrections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDictionary {
    entries: HashMap<String, String>,
}

impl PersonalDictionary {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Add a correction: when Whisper outputs `from`, replace with `to`
    pub fn add_entry(&mut self, from: String, to: String) {
        self.entries.insert(from.to_lowercase(), to);
    }

    pub fn remove_entry(&mut self, from: &str) {
        self.entries.remove(&from.to_lowercase());
    }

    /// Apply all dictionary corrections to the text
    pub fn apply_corrections(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (from, to) in &self.entries {
            // Case-insensitive word boundary replacement
            let pattern = format!(r"(?i)\b{}\b", regex::escape(from));
            if let Ok(re) = regex::Regex::new(&pattern) {
                result = re.replace_all(&result, to.as_str()).to_string();
            }
        }
        result
    }
}

impl Default for PersonalDictionary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrects_known_terms() {
        let mut dict = PersonalDictionary::new();
        dict.add_entry("whisper nose".to_string(), "Whipernoz".to_string());

        let result = dict.apply_corrections("I'm working on whisper nose project");
        assert_eq!(result, "I'm working on Whipernoz project");
    }

    #[test]
    fn case_insensitive_matching() {
        let mut dict = PersonalDictionary::new();
        dict.add_entry("claude".to_string(), "Claude".to_string());

        let result = dict.apply_corrections("I use CLAUDE for text cleanup");
        assert_eq!(result, "I use Claude for text cleanup");
    }
}
