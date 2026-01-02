use common::{
    backend::{Backend, BackendVersion, DisplayBoundsInfo, DisplayModeInfo},
    vs_error::VSError,
};
use log::{error, info, warn};
use sdl2::Sdl;

#[allow(non_camel_case_types)]
pub struct SDL2_Backend {
    sdl_context: Sdl,
}

impl Backend for SDL2_Backend {
    fn new() -> Self {
        info!("Initializing SDL");
        let sdl_context = match sdl2::init() {
            Ok(context) => context,
            Err(err) => panic!("Could not initialize SDL: {err}"),
        };

        SDL2_Backend { sdl_context }
    }

    fn init(&mut self) {}

    fn version(&self) -> common::backend::BackendVersion {
        let vers = sdl2::version::version();
        BackendVersion {
            backend_name: "SDL2",
            major: vers.major,
            minor: vers.minor,
            patch: vers.patch,
        }
    }

    /// Returns a list of display mode height and widths
    fn display_modes(&self) -> Result<Vec<DisplayModeInfo>, VSError> {
        let Ok(video) = self.sdl_context.video() else {
            error!("no video subsystem in sdl2 backend");
            return Err(VSError::Backend_NoVideo);
        };
        let Ok(num_modes) = video.num_display_modes(0) else {
            error!("unable to determine number of video modes in sdl2 backend");
            return Err(VSError::Backend_NoVideoModes);
        };

        let list = (0..num_modes)
            .filter_map(|md| {
                if let Ok(display_mode) = video.display_mode(0, md) {
                    if (md < 0) || (display_mode.w < 0) || (display_mode.h < 0) {
                        error!(
                            "bad resolution: mode {} is {} w x {} h",
                            md, display_mode.w, display_mode.h
                        );
                        None
                    } else {
                        Some(DisplayModeInfo {
                            mode: md as u16,
                            width: display_mode.w as u16, // checked, above, not negative
                            height: display_mode.h as u16,
                        })
                    }
                } else {
                    warn!("no display mode info for display mode {md}");
                    None
                }
            })
            .collect::<Vec<DisplayModeInfo>>();

        Ok(list)
    }

    /// Returns a list of display mode height and widths
    fn display_bounds(&self) -> Result<Vec<DisplayBoundsInfo>, VSError> {
        let Ok(video) = self.sdl_context.video() else {
            error!("no video subsystem in sdl2 backend");
            return Err(VSError::Backend_NoVideo);
        };
        let Ok(num_displays) = video.num_video_displays() else {
            error!("unable to determine number of video displays in sdl2 backend");
            return Err(VSError::Backend_DisplayBoundsInfoMissing);
        };

        let list = (0..num_displays)
            .filter_map(|disp| {
                if let Ok(bounds) = video.display_bounds(disp) {
                    let name = {
                        if let Ok(name) = video.display_name(disp) {
                            name
                        } else {
                            warn!("no name for display {disp}");
                            String::new()
                        }
                    };
                    if (bounds.w < 0) || (bounds.h < 0) {
                        error!(
                            "bad display bounds: display {} is {} w x {} h",
                            disp, bounds.w, bounds.h
                        );
                        None
                    } else {
                        Some(DisplayBoundsInfo {
                            name,
                            width: bounds.w as u16, // checked, above, not negative
                            height: bounds.h as u16,
                        })
                    }
                } else {
                    warn!("no display mode info for display mode {disp}");
                    None
                }
            })
            .collect::<Vec<DisplayBoundsInfo>>();

        Ok(list)
    }

    fn show_cursor(&mut self, show: bool) {
        self.sdl_context.mouse().show_cursor(show);
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
