/// Voice command types that can be detected in transcribed text
#[derive(Debug, Clone, PartialEq)]
pub enum VoiceCommand {
    MakeProfessional,
    MakeCasual,
    TurnIntoBulletList,
    TurnIntoEmail,
    FixGrammar,
    DeleteLastSentence,
    Undo,
    Custom(String),
}

/// Known command trigger phrases
const COMMAND_TRIGGERS: &[(&str, fn() -> VoiceCommand)] = &[
    ("make it professional", || VoiceCommand::MakeProfessional),
    ("make it more professional", || VoiceCommand::MakeProfessional),
    ("reformule en pro", || VoiceCommand::MakeProfessional),
    ("make it casual", || VoiceCommand::MakeCasual),
    ("make it more casual", || VoiceCommand::MakeCasual),
    ("turn into bullet list", || VoiceCommand::TurnIntoBulletList),
    ("fais une liste", || VoiceCommand::TurnIntoBulletList),
    ("turn into email", || VoiceCommand::TurnIntoEmail),
    ("fix grammar", || VoiceCommand::FixGrammar),
    ("corrige la grammaire", || VoiceCommand::FixGrammar),
    ("delete last sentence", || VoiceCommand::DeleteLastSentence),
    ("supprime la dernière phrase", || VoiceCommand::DeleteLastSentence),
    ("undo", || VoiceCommand::Undo),
    ("annuler", || VoiceCommand::Undo),
];

/// Try to detect a voice command in the transcribed text
pub fn detect_command(transcript: &str) -> Option<VoiceCommand> {
    let lower = transcript.to_lowercase();
    for (trigger, make_cmd) in COMMAND_TRIGGERS {
        if lower.contains(trigger) {
            return Some(make_cmd());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_professional_command() {
        assert_eq!(
            detect_command("make it professional"),
            Some(VoiceCommand::MakeProfessional)
        );
    }

    #[test]
    fn detects_french_command() {
        assert_eq!(
            detect_command("fais une liste"),
            Some(VoiceCommand::TurnIntoBulletList)
        );
    }

    #[test]
    fn returns_none_for_regular_text() {
        assert_eq!(detect_command("I think we should meet on Tuesday"), None);
    }
}
