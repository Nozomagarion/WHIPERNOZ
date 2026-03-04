/// Information about a detected text field
#[derive(Debug, Clone)]
pub struct TextFieldInfo {
    /// Bounding rectangle in screen coordinates
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    /// Name of the application that owns this text field
    pub app_name: String,
    /// Window title
    pub window_title: String,
    /// Whether the field is editable
    pub is_editable: bool,
}

/// Trait for platform-specific text field detection
pub trait TextFieldDetector {
    /// Start watching for focused text field changes
    fn start_watching(&mut self) -> anyhow::Result<()>;
    /// Stop watching
    fn stop_watching(&mut self) -> anyhow::Result<()>;
    /// Get the currently focused text field, if any
    fn get_focused_field(&self) -> Option<TextFieldInfo>;
}
