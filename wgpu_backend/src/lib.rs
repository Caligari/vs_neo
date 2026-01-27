use std::sync::Arc;

use common::backend;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

pub mod renderer_wgpu;

#[allow(non_camel_case_types)]
pub struct WGPU_Backend {
    window: Window,
    device: Device,
    queue: Queue,
    surface: Surface,
    config: SurfaceConfiguration,
}

impl backend::Backend for WGPU_Backend {
    fn new() -> Self
    where
        Self: Sized,
    {
    }

    fn init(&mut self) {}

    fn version(&self) -> backend::BackendVersion {}

    fn display_modes(&self) -> Result<Vec<backend::DisplayModeInfo>, common::vs_error::VSError> {}

    fn display_bounds(&self) -> Result<Vec<backend::DisplayBoundsInfo>, common::vs_error::VSError> {
    }

    fn create_window(
        &mut self,
        width: common::window::WindowExtent,
        height: common::window::WindowExtent,
        depth: common::window::WindowDepth,
        window_type: common::window::WindowType,
        _buffer_count: common::window::WindowBuffers,
        _antialiass: bool,
        _vsyncnc: bool,
    ) -> Result<(), common::vs_error::VSError> {
    }

    fn show_cursor(&mut self, show: bool) {}
}
