use std::sync::OnceLock;

use log::info;
use sdl2_sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency};

use crate::SharedTimeVal;

static LOGGING: OnceLock<bool> = OnceLock::new();

pub fn start_timer_log(timer: SharedTimeVal) {
    if LOGGING.set(true).is_ok() {
        let target_width = 28;
        let time_precision = 4;
        let pre_time_width = time_precision + 2;
        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{} [{:>target_width$}] [{}] {}",
                    match *timer.lock().expect("unable to get time in log") {
                        Some(t) => {
                            let ts = unsafe {
                                ((SDL_GetPerformanceCounter() * 1000000)
                                    / SDL_GetPerformanceFrequency())
                                    - t
                            } as f32
                                / 1000000.0;
                            format!("{:.time_precision$}", ts)
                        }
                        None => format!("{:pre_time_width$}", " "),
                    },
                    {
                        let s = record.target();
                        match s.strip_prefix("vectorstorm::") {
                            Some(s) => s,
                            _ => s,
                        }
                    },
                    record.level(),
                    message,
                ))
            })
            .level(log::LevelFilter::Debug)
            .chain(std::io::stdout())
            .chain(fern::log_file("log.txt").expect("unable to create log file"))
            .apply()
            .expect("unable to start logger");
    } else {
        info!("Log already started");
    }
}
