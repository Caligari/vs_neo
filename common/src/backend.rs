use crate::vs_error::VSError;

/// Defines the api for the backend
///

pub trait Backend {
    fn new() -> Self
    where
        Self: Sized;
    fn init(&mut self);

    fn display_modes(&self) -> Result<Vec<DisplayModeInfo>, VSError>;
    fn show_cursor(&mut self, show: bool);
}

pub struct DisplayModeInfo {
    pub mode: u16,
    pub width: u16,
    pub height: u16,
}
