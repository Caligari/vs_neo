use common::backend::Backend;
use common::vs_error::VSError;
use common::window::WindowExtent;
use core::fmt::Debug;
use indexmap::IndexSet;
use log::info;
use log::warn;

#[allow(unused_imports)]
use common::utils::preferences::PreferenceNumber;
use common::utils::preferences::Preferences;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Resolution {
    width: WindowExtent,
    height: WindowExtent,
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

    pub fn get_resolution_extents(&self) -> (WindowExtent, WindowExtent) {
        let res = self
            .supported_resolutions
            .get(
                self.selected_resolution
                    .expect("system has no selected resolution"),
            )
            .expect("system unable to find selected resolution");
        (res.width as WindowExtent, res.height as WindowExtent)
    }

    pub fn get_window_resolution_xy(&self) -> (WindowExtent, WindowExtent) {
        (
            self.preferences
                .get_number_preference(WINDOW_RESOLUTION_X)
                .expect("system unable to find window resolution x setting")
                as WindowExtent,
            self.preferences
                .get_number_preference(WINDOW_RESOLUTION_Y)
                .expect("system unable to find window resolution y setting")
                as WindowExtent,
        )
    }

    pub fn discover_resolutions(&mut self, backend: &Box<dyn Backend>) -> Result<(), VSError> {
        info!("Checking supported resolutions...");
        match backend.display_modes() {
            Ok(display_modes) => {
                // find w and h max and best resolution mode
                let (max_width, max_height, best_res) = {
                    let mut max_w = 0;
                    let mut max_h = 0;
                    let mut best_res = None;

                    self.supported_resolutions = display_modes
                        .iter()
                        .map(|display_mode_info| {
                            let res = Resolution {
                                width: display_mode_info.width,
                                height: display_mode_info.height,
                            };
                            if (res.width >= max_w) && (res.height >= max_h) {
                                best_res = Some(res);
                            }
                            max_w = max_w.max(res.width);
                            max_h = max_h.max(res.height);
                            res
                        })
                        .collect::<IndexSet<Resolution>>()
                        .drain(..)
                        .collect();
                    // println!("{:?}", self.supported_resolutions);
                    (max_w as WindowExtent, max_h as WindowExtent, best_res)
                };

                self.preferences.constrain_number_preference(
                    WINDOW_RESOLUTION_X,
                    1280,
                    0,
                    max_width,
                );
                self.preferences.constrain_number_preference(
                    WINDOW_RESOLUTION_Y,
                    720,
                    0,
                    max_height,
                );

                // record which resolution to use
                if let Some(using_res) = best_res {
                    if let Some(res_num) = self
                        .supported_resolutions
                        .iter()
                        .position(|r| *r == using_res)
                    {
                        self.preferences
                            .set_number_preference(RESOLUTION_X, using_res.width);
                        self.preferences
                            .set_number_preference(RESOLUTION_Y, using_res.height);
                        self.selected_resolution = Some(res_num);

                        info!(
                            "Selected resolution {} (width: {}, height: {})",
                            res_num, using_res.width, using_res.height
                        );

                        Ok(())
                    } else {
                        Err(VSError::Backend_NoResolutionFound)
                    }
                } else {
                    // no resolution found
                    Err(VSError::Backend_NoResolutionFound)
                }
            }

            Err(e) => {
                warn!("no display modes discovered");
                self.supported_resolutions = Vec::new();
                self.selected_resolution = None;
                Err(e)
            }
        }
    }
}

impl Default for SystemPreferences {
    fn default() -> Self {
        SystemPreferences::new()
    }
}
