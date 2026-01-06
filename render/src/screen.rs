use common::{
    backend::Backend,
    window::{WindowBuffers, WindowDepth, WindowExtent, WindowType},
};
use log::{debug, error, info};

use super::{display_list::DisplayList, renderer::Renderer, renderer_opengl3::RendererOpenGL3};

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
    // sdl: Rc<Sdl>,
    renderer: RendererOpenGL3, // how can we make this more independant?
}

impl Screen {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        backend: &mut Box<dyn Backend>,
        width: WindowExtent,
        height: WindowExtent,
        depth: WindowDepth,
        window_type: WindowType,
        buffer_count: WindowBuffers,
        vsync: bool,
        antialias: bool,
        _high_dpi: bool,
    ) -> Self {
        info!("Creating Screen...");

        debug!("Screen width before create renderer: {}", width);
        let renderer = RendererOpenGL3::new(
            backend,
            width,
            height,
            depth,
            window_type,
            buffer_count,
            antialias,
            vsync,
        );

        let (new_width, new_height) = if let Some(renderer_data) = renderer.get_render_data() {
            let new_width = renderer_data.borrow().width;
            let new_height = renderer_data.borrow().height;

            (new_width, new_height)
        } else {
            error!("no render data available, when creating Screen");
            (1, 1) // bad default
        };

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
            renderer,
        }
    }
}
