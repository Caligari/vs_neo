use sdl2_sys::{SDL_Delay, SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency};
use std::sync::{Arc, Mutex};

use crate::core::{core_game::CoreGame, core_game_system::CoreGameSystem};
use common::{SharedTimeStep, SharedTimeVal};

// type LaunchTime = SharedTimeVal;

#[allow(dead_code)]
#[derive(Default)]
pub struct TimerSystem {
    active: bool,

    launch_time: SharedTimeVal,
    init_time: u64,

    start_cpu: u64,
    start_gather: u64,
    // start_draw: u64,
    // start_gpu: u64,
    missed_frames: u32, // ?? u16

    // cpu_time: u64,
    // gather_time: u64,
    // draw_time: u64,
    // gpu_time: u64,
    first_frame: bool,

    time_step: SharedTimeStep,

    refresh_rate: Arc<Mutex<u16>>,
}

impl CoreGameSystem for TimerSystem {
    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn init(&mut self) {
        let microseconds = get_microseconds();

        self.init_time = microseconds;
        self.start_cpu = microseconds;
        self.start_gather = microseconds;
        // self.start_draw = microseconds;
        // self.start_gpu = microseconds;
        self.missed_frames = 0;
        self.first_frame = true;
    }

    fn deinit(&mut self) {}

    fn update(&mut self, _core: &mut CoreGame) {
        let (now, round_time) = {
            let n = get_microseconds();
            let max_fps = *self
                .refresh_rate
                .lock()
                .expect("unable to unlock refresh rate in timer update");
            let desired_ticks_per_round = 1000000 / max_fps as u64;
            let min_ticks_per_round = desired_ticks_per_round / 2;
            {
                let rt = {
                    let rt = n - self.start_cpu;
                    if rt > 100000 {
                        self.start_cpu = n - desired_ticks_per_round; // !! can this go negative?
                        self.start_cpu
                    } else {
                        rt
                    }
                };

                // enforce fps maximum
                if rt < min_ticks_per_round {
                    let delay_ticks = desired_ticks_per_round / 1000;
                    unsafe {
                        SDL_Delay(
                            delay_ticks
                                .try_into()
                                .expect("too many delay ticks in timer system update"),
                        );
                    }
                    let n = get_microseconds();
                    (n - self.start_cpu, n)
                } else {
                    (n, rt)
                }
            }
        };

        let actual_time_step = {
            const MIN_TIME_PER_FRAME: f32 = 2.0 / 60.0;
            const MAX_TIME_PER_FRAME: f32 = 1.0 / 60.0;

            let ts = round_time as f32 / 1000000_f32;

            if self.first_frame {
                self.first_frame = false;
                MIN_TIME_PER_FRAME
            } else if ts > MAX_TIME_PER_FRAME {
                self.missed_frames += 1;
                MAX_TIME_PER_FRAME
            } else {
                ts
            }
        };

        *self
            .time_step
            .lock()
            .expect("unable to unlock time step in timer update") = actual_time_step;
        self.start_cpu = now;
    }

    fn post_update(&mut self, _core: &mut CoreGame) {
        self.start_gather = get_microseconds();
    }
}

#[allow(dead_code)]
impl TimerSystem {
    pub fn new(refresh_rate: Arc<Mutex<u16>>, launch_time: SharedTimeVal) -> Self {
        let microseconds = get_microseconds();
        *launch_time
            .lock()
            .expect("unable to lock launch time on timer system new") = Some(microseconds);

        TimerSystem {
            active: true,
            launch_time,
            init_time: microseconds,
            time_step: Arc::new(Mutex::new(0.0)),
            refresh_rate,
            ..Default::default()
        }
    }

    pub fn get_time_step_ref(&self) -> Arc<Mutex<f32>> {
        Arc::clone(&self.time_step)
    }

    fn get_microseconds_since_init(&self) -> u64 {
        get_microseconds() - self.init_time
    }

    fn get_microseconds_since_launch(&self) -> u64 {
        if let Some(launch) = *self
            .launch_time
            .lock()
            .expect("unable to lock launch time in timer system get time since launch")
        {
            get_microseconds() - launch
        } else {
            0
        }
    }
}

fn get_microseconds() -> u64 {
    unsafe {
        let counter = SDL_GetPerformanceCounter();
        (counter * 1000000) / SDL_GetPerformanceFrequency()
    }
}
