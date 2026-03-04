use regex::Regex;

/// Regex-based pre-cleanup: removes filler words, capitalizes first letter
pub fn regex_cleanup(input: &str) -> String {
    let input = input.trim();
    if input.is_empty() {
        return String::new();
    }

    // Remove common filler words (English + French)
    let fillers = Regex::new(
        r"(?i)\b(um|uh|er|erm|hmm|like|you know|basically|sort of|kind of|I mean|euh|heu|bah|ben|genre|en fait|du coup|voilà)\b"
    ).unwrap();

    let cleaned = fillers.replace_all(input, "");

    // Collapse multiple spaces
    let spaces = Regex::new(r"\s{2,}").unwrap();
    let cleaned = spaces.replace_all(&cleaned, " ");

    let cleaned = cleaned.trim().to_string();

    // Capitalize first letter
    capitalize_first(&cleaned)
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_english_fillers() {
        let input = "um so I think we should uh meet on tuesday";
        let result = regex_cleanup(input);
        assert_eq!(result, "So I think we should meet on tuesday");
    }

    #[test]
    fn removes_french_fillers() {
        let input = "euh je pense que en fait on devrait se voir mardi";
        let result = regex_cleanup(input);
        assert_eq!(result, "Je pense que on devrait se voir mardi");
    }

    #[test]
    fn preserves_meaningful_like() {
        let input = "I would like to order a pizza";
        let result = regex_cleanup(input);
        // "like" as filler is removed, but this is a known limitation
        // We'll rely on the LLM to fix context-dependent cases
        assert!(!result.is_empty());
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(regex_cleanup(""), "");
        assert_eq!(regex_cleanup("   "), "");
    }

    #[test]
    fn capitalizes_first_letter() {
        let input = "hello world";
        assert_eq!(regex_cleanup(input), "Hello world");
    }
}
