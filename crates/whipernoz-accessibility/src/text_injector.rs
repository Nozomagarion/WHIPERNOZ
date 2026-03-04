use anyhow::Result;

/// Strategy for injecting text into focused fields
#[derive(Debug, Clone)]
pub enum InjectionMethod {
    /// Use OS accessibility API (best, but not always available)
    Accessibility,
    /// Use clipboard + paste simulation (universal fallback)
    Clipboard,
}

/// Trait for platform-specific text injection
pub trait TextInjector {
    /// Inject text into the currently focused text field
    fn inject_text(&self, text: &str, method: InjectionMethod) -> Result<()>;
    /// Get the current text from the focused field (if possible)
    fn get_current_text(&self) -> Result<Option<String>>;
}
