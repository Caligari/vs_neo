use core::fmt::Debug;
use indexmap::IndexSet;
use log::info;
use sdl2::VideoSubsystem;

use common::utils::preferences::PreferenceNumber;
use common::utils::preferences::Preferences;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Resolution {
    width: PreferenceNumber,
    height: PreferenceNumber,
}

pub struct SystemPreferences {
    preferences: Preferences,
    supported_resolutions: Vec<Resolution>,
    selected_resolution: Option<usize>,
}

const FULLSCREEN: &str = "Fullscreen";
const FULLSCREEN_WINDOW: &str = "FullscreenWindow";
const VSYNC: &str = "VSync";
const BLOOM: &str = "Bloom";
const DYNAMIC_BATCHING: &str = "DynamicBatching";
const ANTIALIAS: &str = "Antialias";
const HIGH_DPI: &str = "HighDPI";
const EFFECT_VOLUME: &str = "EffectVolume";
const MUSIC_VOLUME: &str = "MusicVolume";
const WHEEL_SMOOTHING: &str = "WheelSmoothing";
const MOUSE_WHEEL_SCALE_PERCENT: &str = "MouseWheelScalePercent";
const TRACKPAD_WHEEL_SCALE_PERCENT: &str = "TrackpadWheelScalePercent";
const WINDOW_RESOLUTION_X: &str = "WindowResolutionX";
const WINDOW_RESOLUTION_Y: &str = "WindowResolutionY";
const RESOLUTION_X: &str = "ResolutionX";
const RESOLUTION_Y: &str = "ResolutionY";

impl SystemPreferences {
    pub fn new() -> Self {
        let mut prefs = Preferences::new("vectorstorm");
        prefs.constrain_boolean_preference(FULLSCREEN, true);
        prefs.constrain_boolean_preference(FULLSCREEN_WINDOW, true);
        prefs.constrain_boolean_preference(VSYNC, true);
        prefs.constrain_boolean_preference(BLOOM, true);
        prefs.constrain_boolean_preference(DYNAMIC_BATCHING, true);
        prefs.constrain_boolean_preference(ANTIALIAS, false);
        prefs.constrain_boolean_preference(HIGH_DPI, false);
        prefs.constrain_number_preference(EFFECT_VOLUME, 100, 0, 100);
        prefs.constrain_number_preference(MUSIC_VOLUME, 100, 0, 100);
        prefs.constrain_boolean_preference(WHEEL_SMOOTHING, true);
        prefs.constrain_number_preference(MOUSE_WHEEL_SCALE_PERCENT, 100, 0, 10000);
        prefs.constrain_number_preference(TRACKPAD_WHEEL_SCALE_PERCENT, 10, 0, 10000);

        SystemPreferences {
            preferences: prefs,
            supported_resolutions: Vec::new(),
            selected_resolution: None,
        }
    }

    pub fn get_fullscreen(&self) -> bool {
        self.preferences
            .get_boolean_preference(FULLSCREEN)
            .expect("system unable to find fullscreen setting")
    }

    pub fn get_fullscreen_window(&self) -> bool {
        self.preferences
            .get_boolean_preference(FULLSCREEN_WINDOW)
            .expect("system unable to find fullscreen window setting")
    }

    pub fn get_bloom(&self) -> bool {
        self.preferences
            .get_boolean_preference(BLOOM)
            .expect("system unable to find bloom setting")
    }

    pub fn get_vsync(&self) -> bool {
        self.preferences
            .get_boolean_preference(VSYNC)
            .expect("system unable to find vsync setting")
    }

    pub fn get_antialias(&self) -> bool {
        self.preferences
            .get_boolean_preference(ANTIALIAS)
            .expect("system unable to find antialias setting")
    }

    pub fn get_high_dpi(&self) -> bool {
        self.preferences
            .get_boolean_preference(HIGH_DPI)
            .expect("system unable to find high dpi setting")
    }

    pub fn get_resolution_extents(&self) -> (PreferenceNumber, PreferenceNumber) {
        let res = self
            .supported_resolutions
            .get(
                self.selected_resolution
                    .expect("system has no selected resolution"),
            )
            .expect("system unable to find selected resolution");
        (res.width, res.height)
    }

    pub fn get_window_resolution_xy(&self) -> (PreferenceNumber, PreferenceNumber) {
        (
            self.preferences
                .get_number_preference(WINDOW_RESOLUTION_X)
                .expect("system unable to find window resolution x setting"),
            self.preferences
                .get_number_preference(WINDOW_RESOLUTION_Y)
                .expect("system unable to find window resolution y setting"),
        )
    }

    pub fn check_resolutions(&mut self, video: &VideoSubsystem) {
        info!("Checking supported resolutions...");

        // find w and h max and best resolution mode
        let (max_width, max_height, best_res) = {
            let num_modes = video
                .num_display_modes(0)
                .expect("unable to discover number of video modes in system check resolutions"); // !! assumes display 0
            let mut resolutions = IndexSet::new();
            let mut max_w = 0;
            let mut max_h = 0;
            let mut best_res = None;

            for mode in 0..num_modes {
                let d_m = video
                    .display_mode(0, mode)
                    .expect("unable to get display mode info during system check resolutions");
                let md = resolutions.len();
                if resolutions.insert(Resolution {
                    width: d_m.w as PreferenceNumber,
                    height: d_m.h as PreferenceNumber,
                }) {
                    // only inserted if unique
                    if (d_m.w >= max_w) && (d_m.h >= max_h) {
                        best_res = Some(md);
                    }
                    max_w = max_w.max(d_m.w);
                    max_h = max_h.max(d_m.h);
                }
            }
            self.supported_resolutions = resolutions.drain(..).collect(); // save resolution table
            // println!("{:?}", self.supported_resolutions);
            (
                max_w as PreferenceNumber,
                max_h as PreferenceNumber,
                best_res,
            )
        };

        self.preferences
            .constrain_number_preference(WINDOW_RESOLUTION_X, 1280, 0, max_width);
        self.preferences
            .constrain_number_preference(WINDOW_RESOLUTION_Y, 720, 0, max_height);

        // record which resolution to use
        let using_res =
            best_res.expect("did not find usable resolution when system checking resolutions");
        let res = self
            .supported_resolutions
            .get(using_res)
            .expect("unable to find selected resolution during system check resolutions");

        self.preferences
            .set_number_preference(RESOLUTION_X, res.width);
        self.preferences
            .set_number_preference(RESOLUTION_Y, res.height);
        self.selected_resolution = best_res;

        info!(
            "Selected resolution {} (width: {}, height: {})",
            using_res, res.width, res.height
        );
    }
}

impl Default for SystemPreferences {
    fn default() -> Self {
        SystemPreferences::new()
    }
}
