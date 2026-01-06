use std::fmt::Display;

use crate::{
    vs_error::VSError,
    window::{WindowBuffers, WindowDepth, WindowExtent, WindowType},
};

/// Defines the api for the backend
///

pub trait Backend {
    fn new() -> Self
    where
        Self: Sized;
    fn init(&mut self);

    fn version(&self) -> BackendVersion;
    fn display_modes(&self) -> Result<Vec<DisplayModeInfo>, VSError>;
    fn display_bounds(&self) -> Result<Vec<DisplayBoundsInfo>, VSError>;

    fn create_window(
        &mut self,
        width: WindowExtent,
        height: WindowExtent,
        depth: WindowDepth,
        window_type: WindowType,
        _buffer_count: WindowBuffers,
        _antialiass: bool,
        _vsyncnc: bool,
    ) -> Result<(), VSError>;

    fn show_cursor(&mut self, show: bool);
}

#[derive(Debug, Clone, Copy)]
pub struct BackendVersion {
    pub backend_name: &'static str,
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl Display for BackendVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Runtime {}.{}.{}",
            self.backend_name, self.major, self.minor, self.patch
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DisplayModeInfo {
    pub mode: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub struct DisplayBoundsInfo {
    pub name: String,
    pub width: u16,
    pub height: u16,
}
