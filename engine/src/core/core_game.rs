use crate::{
    RefGameSystem, RefTimerSystem,
    core::{core_game_mode::*, core_game_system::CoreGameSystem, vs_core::GameSystemMap},
};
use input::RefInputSystem;
use log::info;
use physics::RefCollisionSystem;
use sound::RefSoundSystem;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum GameSystem {
    Timer(RefTimerSystem),
    Input(RefInputSystem),
    Collision(RefCollisionSystem),
    Sound(RefSoundSystem),
}

pub trait GameCode {
    fn update(&mut self, delta: f32);
    fn draw_frame(&mut self);
    fn should_exit(&self) -> bool;
}

#[allow(dead_code)]
pub struct CoreGame {
    // this is likely a trait or contains a reference to one?
    pub name: String,

    // are these single-threaded? Do we need mutex?
    game_system_order: Vec<RefGameSystem>, // [GameSystem; GameSystem::COUNT],  // alternate order for system

    frames_rendered: u64,
    start_ticks: u64,

    time_step: Arc<Mutex<f32>>, // comes from the TimerSystem, which must exist
    exit: bool,
    first_frame: bool,

    current_mode: Option<CoreGameMode>,
    scene_count: u32,

    code: Box<dyn GameCode>,
}

impl CoreGame {
    pub fn new(name: &str, code: Box<dyn GameCode>, systems: &GameSystemMap) -> Self {
        let mut t_step = None;
        for s in systems.clone().iter() {
            match &*s.borrow_mut() {
                GameSystem::Timer(t) => {
                    let timer = t.borrow_mut();
                    let ts = timer.get_time_step_ref();
                    t_step = Some(ts);
                }
                _ => (),
            }
        }
        CoreGame {
            name: name.to_string(),
            game_system_order: systems.clone(),
            frames_rendered: 0,
            start_ticks: 0,
            time_step: t_step
                .expect("unable to get time_step from timer system on CoreGame creation"),
            exit: false,
            first_frame: false,
            current_mode: None,
            scene_count: 1,
            code,
        }
    }

    pub fn init(&mut self) {}

    pub fn deinit(&mut self) {}

    pub fn start_timer(&mut self) {}

    pub fn stop_timer(&mut self) {}

    pub fn should_exit(&self) -> bool {
        self.exit
    }

    pub fn go(&mut self) {
        self.frames_rendered += 1;

        // update game systems
        for system in self.game_system_order.clone().iter() {
            match &*system.borrow_mut() {
                GameSystem::Timer(t) => {
                    let mut timer = t.borrow_mut();
                    if timer.is_active() {
                        timer.update(self);
                    }
                }

                // TODO: Other systems
                _ => (),
            }
        }

        info!("in core game go");

        self.code.update(
            *self
                .time_step
                .lock()
                .expect("unable to fetch timestep from mutex in core game update"),
        );

        // ..

        // update game systems post update
        for system in self.game_system_order.clone().iter() {
            match &*system.borrow_mut() {
                GameSystem::Timer(t) => {
                    let mut timer = t.borrow_mut();
                    if timer.is_active() {
                        timer.update(self);
                    }
                }
                _ => (),
            }
        }

        self.code.draw_frame();

        self.exit = self.code.should_exit();
    }
}
