use std::{cell::RefCell, rc::Rc};

use common::{
    backend::Backend,
    window::{WindowBuffers, WindowDepth, WindowExtent, WindowType},
};

use super::{display_list::DisplayList, shader_suite::ShaderSuite};

pub trait Renderer: Sized {
    #[allow(clippy::too_many_arguments)]
    fn new(
        backend: &mut Box<dyn Backend>,
        width: WindowExtent,
        height: WindowExtent,
        depth: WindowDepth,
        window_type: WindowType,
        buffer_count: WindowBuffers,
        antialias: bool,
        vsync: bool,
    ) -> Self;
    fn check_video_mode(&self) -> bool;
    #[allow(clippy::too_many_arguments)]
    fn update_video_mode(
        &mut self,
        width: WindowExtent,
        height: WindowExtent,
        depth: WindowDepth,
        window_type: WindowType,
        buffer_count: WindowBuffers,
        antialias: bool,
        vsync: bool,
    );
    // notify_resized ( width: u16, height: u16 );
    fn pre_render(&mut self, settings: &RenderSettings);
    fn render_display_list(&mut self, display_list: &DisplayList);
    // fn raw_render_display_list ( display_list: &DisplayList );
    fn post_render(&mut self);

    fn get_render_data(&self) -> Option<Rc<RefCell<RendererData>>>;
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RenderSettings {
    shader_suite: Option<ShaderSuite>,
    aspect_ratio: f32,
    polygon_offset_units: f32,
    use_custom_aspect_ratio: bool,
    write_color: bool,
    write_depth: bool,
    invert_cull: bool,
}

impl Default for RenderSettings {
    fn default() -> Self {
        RenderSettings {
            shader_suite: None,
            aspect_ratio: 1.0,
            polygon_offset_units: 0.0,
            use_custom_aspect_ratio: false,
            write_color: true,
            write_depth: true,
            invert_cull: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct RendererData {
    pub settings: RenderSettings,
    pub width: WindowExtent,
    pub height: WindowExtent,
    pub viewport_width: WindowExtent,
    pub viewport_height: WindowExtent,

    pub width_pixels: WindowExtent,
    pub height_pixels: WindowExtent,
    pub viewport_width_pixels: WindowExtent,
    pub viewport_height_pixels: WindowExtent,

    pub refresh_rate: u16, // ?? could this be u8
}

impl RendererData {
    pub fn get_current_settings(&self) -> &RenderSettings {
        &self.settings
    }

    // TODO as needed
}
