use common::{
    backend::{Backend, DisplayModeInfo},
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

    /// Returns a list of display mode height and widths
    fn display_modes(&self) -> Result<Vec<DisplayModeInfo>, VSError> {
        let Ok(video) = self.sdl_context.video() else {
            error!("no video subsystem in sdl backend");
            return Err(VSError::Backend_NoVideo);
        };
        let Ok(num_modes) = video.num_display_modes(0) else {
            error!("unable to determine number of video modes in sdl backend");
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
