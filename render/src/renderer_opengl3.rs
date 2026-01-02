use common::backend::Backend;
use log::{error, info, warn};
use sdl2::{Sdl, video::GLProfile};
use sdl2_sys::SDL_WindowFlags;
use std::{cell::RefCell, rc::Rc};

use super::renderer::{
    Renderer, RendererData, WindowBuffers, WindowDepth, WindowExtent, WindowType,
};

#[allow(dead_code)]
pub struct RendererOpenGL3 {
    renderer_data: Rc<RefCell<RendererData>>,
    window: sdl2::video::Window,
    // sdl: Rc<Sdl>,
}

impl Renderer for RendererOpenGL3 {
    fn new(
        backend: &Box<dyn Backend>,
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

        let (renderer_data, window) = RendererOpenGL3::create_window(
            sdl.clone(),
            width,
            height,
            depth,
            window_type,
            buffer_count,
            antialias,
            vsync,
        );

        RendererOpenGL3 {
            renderer_data: Rc::new(RefCell::new(renderer_data)),
            window,
            sdl,
        }
    }

    fn check_video_mode(&self) -> bool {
        // ?? used?
        false
    }

    fn update_video_mode(
        &mut self,
        _width: super::renderer::WindowExtent,
        _height: super::renderer::WindowExtent,
        _depth: super::renderer::WindowDepth,
        _window_type: super::renderer::WindowType,
        _buffer_count: super::renderer::WindowBuffers,
        _antialiass: bool,
        _vsync: bool,
    ) {
    }

    fn get_render_data(&self) -> Rc<RefCell<RendererData>> {
        self.renderer_data.clone()
    }

    fn pre_render(&mut self, _settings: &super::renderer::RenderSettings) {}

    fn post_render(&mut self) {}

    fn render_display_list(&mut self, _display_list: &super::display_list::DisplayList) {}
}

impl RendererOpenGL3 {
    #[allow(clippy::too_many_arguments)]
    fn create_window(
        backend: &Box<dyn Backend>,
        width: WindowExtent,
        height: WindowExtent,
        depth: WindowDepth,
        window_type: WindowType,
        _buffer_count: WindowBuffers,
        _antialiass: bool,
        _vsyncnc: bool,
    ) -> (RendererData, sdl2::video::Window) {
        let video_subsystem = &sdl
            .video()
            .expect("unable to find sdl video subsystem in openGL3 renderer creation");

        let video_flags = SDL_WindowFlags::SDL_WINDOW_OPENGL as u32
            | match window_type {
                WindowType::Fullscreen => {
                    info!("videoFlag added: Fullscreen");
                    SDL_WindowFlags::SDL_WINDOW_FULLSCREEN
                }
                WindowType::FullscreenWindow => {
                    info!("videoFlag added: Fullscreen Desktop");
                    SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP
                }
                WindowType::Window => {
                    info!("videoFlag added: Resizable");
                    SDL_WindowFlags::SDL_WINDOW_RESIZABLE
                }
            } as u32;

        let attributes = video_subsystem.gl_attr();
        attributes.set_double_buffer(true);
        attributes.set_red_size(8);
        attributes.set_green_size(8);
        attributes.set_blue_size(8);
        attributes.set_alpha_size(8);
        attributes.set_depth_size(0); // no depth buffer on our output target - we don't render to it directly
        attributes.set_context_major_version(3);
        attributes.set_context_minor_version(3);
        attributes.set_context_profile(GLProfile::Core);
        attributes.set_share_with_current_context(true);

        info!(
            "SDL_CreateWindow {}x{}x{} video mode, flags {}",
            width, height, depth, video_flags
        );
        let sdl_window = video_subsystem
            .window("", width as u32, height as u32)
            .set_window_flags(video_flags)
            .build()
            .expect("unable to build window during openGL3 renderer creation");

        let renderer_data = RendererData {
            // TODO: will need to be mut
            width,
            height,
            viewport_width: width,
            viewport_height: height,
            refresh_rate: 60,
            ..Default::default()
        };

        (renderer_data, sdl_window)
    }
}
