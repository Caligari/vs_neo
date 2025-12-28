use std::rc::Rc;

use log::{debug, info};
use sdl2::Sdl;

use super::{renderer_opengl3::RendererOpenGL3, renderer::{Renderer, WindowType, WindowExtent, WindowDepth, WindowBuffers}, display_list::DisplayList};


#[allow(dead_code)]
pub struct Screen {
    width: u16,
    height: u16,
    buffer_count: u8,
    depth: u16,
    window_type: WindowType,
    vsync: bool,
    antialias: bool,

    aspect_ratio: f32,

    fifo: DisplayList,

    sdl: Rc<Sdl>,
}

impl Screen {
    #[allow(clippy::too_many_arguments)]
    pub fn new ( sdl: Rc<Sdl>, width: WindowExtent, height: WindowExtent, depth: WindowDepth, window_type: WindowType, buffer_count: WindowBuffers, vsync: bool, antialias: bool, _high_dpi: bool ) -> Self {
        info!("Creating Screen...");

        debug!("Screen width before create renderer: {}", width);
        let renderer = RendererOpenGL3::new(sdl.clone(), width, height, depth, window_type, buffer_count, antialias, vsync);

        let new_width = renderer.get_render_data().borrow().width;
        let new_height = renderer.get_render_data().borrow().height;
        debug!("Width after: {}", new_width);

        let aspect_ratio = new_width as f32 / new_height as f32;
        debug!("Screen Ratio: {}", aspect_ratio);

        let fifo = DisplayList::new();
        // fifo.set_resizable();

        Screen {
            width: new_width,
            height: new_height,
            depth,
            buffer_count,
            window_type,
            vsync,
            antialias,
            aspect_ratio,
            fifo,
            sdl,
        }
    }
}