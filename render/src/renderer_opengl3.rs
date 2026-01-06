use common::{
    backend::Backend,
    window::{WindowBuffers, WindowDepth, WindowExtent, WindowType},
};
use log::{error, info};
use std::{cell::RefCell, rc::Rc};

use super::renderer::{Renderer, RendererData};

#[allow(dead_code)]
pub struct RendererOpenGL3 {
    renderer_data: Option<Rc<RefCell<RendererData>>>,
    // window: sdl2::video::Window,
    // sdl: Rc<Sdl>,
}

impl Renderer for RendererOpenGL3 {
    fn new(
        backend: &mut Box<dyn Backend>,
        width: WindowExtent,
        height: WindowExtent,
        depth: WindowDepth,
        window_type: WindowType,
        buffer_count: WindowBuffers,
        antialias: bool,
        vsync: bool,
    ) -> Self {
        // let mut renderer_data = RendererData {
        //     width, height, viewport_width: width, viewport_height: height,
        //     refresh_rate: 60,
        //     ..Default::default()
        // };

        // TODO: can we find the compiled version? Is it a thing?
        let vers = backend.version();
        info!("Backend version: {}", vers);

        // displays
        match backend.display_bounds() {
            Ok(display_bounds) => {
                info!("Found {} displays:", display_bounds.len());
                for (disp, bounds) in display_bounds.iter().enumerate() {
                    info!(
                        "Display #{} {} ({}x{})",
                        disp, bounds.name, bounds.width, bounds.height,
                    );
                }
            }

            Err(e) => {
                error!(
                    "unable to find display bounds, while creating opengl3 renderer: {:?}", // !! Add error display
                    e
                );
            }
        }

        // invalidate_material = false;

        let renderer_data = match backend.create_window(
            width,
            height,
            depth,
            window_type,
            buffer_count,
            antialias,
            vsync,
        ) {
            Ok(()) => {
                let renderer_data = RendererData {
                    // TODO: will need to be mut
                    width,
                    height,
                    viewport_width: width,
                    viewport_height: height,
                    refresh_rate: 60,
                    ..Default::default()
                };
                Some(Rc::new(RefCell::new(renderer_data)))
            }

            Err(e) => {
                error!(
                    "unable to create window, while creating opengl3 renderer: {:?}", // !! Add error display
                    e
                );
                None
            }
        };

        RendererOpenGL3 { renderer_data }
    }

    fn check_video_mode(&self) -> bool {
        // ?? used?
        false
    }

    fn update_video_mode(
        &mut self,
        _width: WindowExtent,
        _height: WindowExtent,
        _depth: WindowDepth,
        _window_type: WindowType,
        _buffer_count: WindowBuffers,
        _antialiass: bool,
        _vsync: bool,
    ) {
    }

    fn get_render_data(&self) -> Option<Rc<RefCell<RendererData>>> {
        self.renderer_data.clone()
    }

    fn pre_render(&mut self, _settings: &super::renderer::RenderSettings) {}

    fn post_render(&mut self) {}

    fn render_display_list(&mut self, _display_list: &super::display_list::DisplayList) {}
}
