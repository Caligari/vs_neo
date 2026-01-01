use std::sync::{Arc, Mutex};
use std::{collections::HashMap, mem::transmute};

use common::backend::Backend;
use log::info;

use render::renderer::WindowType;
use render::screen::Screen;
use sdl2::{mouse::Cursor, mouse::SystemCursor};
use sdl2_backend::SDL2_Backend;
use sdl2_sys::{SDL_CreateSystemCursor, SDL_Cursor};

use crate::core::vs_core::Core;
use crate::system_preferences::SystemPreferences;
use common::math::random::Random;
use common::utils::log::start_timer_log;

// use super::timer_system::LaunchTime;

pub const VS_VERSION: &str = "r0.0.1";

#[allow(dead_code)]
pub struct System {
    show_cursor: bool,
    show_cursor_overridden: bool,
    cursors: HashMap<SystemCursor, Cursor>, // !! these need to be in backend? Or have VS structs
    cursors2: HashMap<SystemCursor, *mut SDL_Cursor>,
    focused: bool,
    visible: bool,
    exit_game_key_enabled: bool,
    exit_application_key_enabled: bool,
    min_buffers: u8,
    // orientation: Orientation,
    title: String,
    screen: Option<Screen>,
    data_is_pristine: bool,

    // launch_time: LaunchTime,
    system_preferences: SystemPreferences,

    // sdl: Rc<Sdl>,
    backend: Box<dyn Backend>,

    pub random: Random,

    pub core: Core,
}

impl System {
    pub fn new(title: &str, min_buffers: u8) -> Self {
        let launch_time = Arc::new(Mutex::new(None));
        start_timer_log(launch_time.clone());

        // task_Init?

        info!("VectorStorm engine version {}", VS_VERSION);

        // filecache.startup()
        // shadercache.startup()
        // shaderuniformregistry.startup()

        // no need to do here, do in struct creation, below
        // let backend = Box::new(SDL2_Backend::new());

        // init_phys_file_system(.. args ..)
        // preferences

        System {
            show_cursor: true,
            show_cursor_overridden: false,
            cursors: HashMap::new(),
            cursors2: HashMap::new(),
            focused: true,
            visible: false,
            exit_game_key_enabled: true,
            exit_application_key_enabled: true,
            min_buffers,
            title: title.to_string(),
            screen: None,
            data_is_pristine: false,
            system_preferences: SystemPreferences::new(),
            // time_since_launch: time_since_launch.clone(),  // do we need to store this, if we pass it away immediately?
            // sdl: Rc::new(sdl_context),
            backend: Box::new(SDL2_Backend::new()),
            random: Random::new(),
            core: Core::new(launch_time.clone()),
        }
    }

    pub fn init(&mut self) {
        // cursors (do these need to be pre-set?)
        for curs in [
            SystemCursor::Arrow,
            SystemCursor::IBeam,
            SystemCursor::Wait,
            SystemCursor::Hand,
        ] {
            // self.cursors.insert(curs, Cursor::from_system(curs).expect("unable to fetch sdl system cursor on system init"));
            unsafe {
                self.cursors2
                    .insert(curs, SDL_CreateSystemCursor(transmute(curs as u32)));
            }
        }

        // resolution
        self.system_preferences.discover_resolutions(&self.backend);

        let (width, height) = if self.system_preferences.get_fullscreen() {
            self.system_preferences.get_resolution_extents()
        } else {
            self.system_preferences.get_window_resolution_xy()
        };

        if self.system_preferences.get_fullscreen_window() {
            info!("Init:  Initialising fullscreen window");
        } else {
            info!(
                "Init:  Initialising [{}x{}] resolution ({})...",
                width,
                height,
                if self.system_preferences.get_fullscreen() {
                    "fullscreen"
                } else {
                    "windowed"
                }
            ); // LOG
        }

        // get show cursor from get fullscreen preference
        self.show_cursor = !self.system_preferences.get_fullscreen();
        // self.sdl.mouse().show_cursor(self.show_cursor);
        self.backend.show_cursor(self.show_cursor);

        // TODO: texture manager

        // window type
        let window_type = if self.system_preferences.get_fullscreen() {
            if self.system_preferences.get_fullscreen_window() {
                WindowType::FullscreenWindow
            } else {
                WindowType::Fullscreen
            }
        } else {
            WindowType::Window
        };

        // create screen
        let buffer_count = self
            .min_buffers
            .max(if self.system_preferences.get_bloom() {
                2
            } else {
                1
            });
        self.screen = Some(Screen::new(
            self.sdl.clone(), // need to pass backend here
            width,
            height,
            32,
            window_type,
            buffer_count,
            self.system_preferences.get_vsync(),
            self.system_preferences.get_antialias(),
            self.system_preferences.get_high_dpi(),
        ));

        // TODO: log system details

        // TODO: built in font

        // TODO: drag and drop:
        // initialize windows ole
        // Get the hwnd from the SDL window
        // extern SDL_Window *g_sdlWindow;
        // SDL_SysWMinfo wmInfo;
        // SDL_VERSION(&wmInfo.version);
        // SDL_GetWindowWMInfo(g_sdlWindow, &wmInfo);
        // HWND hwnd = wmInfo.info.win.window;
        // create drag and drop targets and register

        // core
        self.core.init();
    }

    pub fn deinit(&mut self) {
        // core
        self.core.deinit();
    }
}

impl Drop for System {
    fn drop(&mut self) {
        // preferences
        // screen
        // deinit_phys_file_system
        // shaderuniformregistry
        // shadercache
        // filecache

        // SDL - done on sdl destruction
    }
}
