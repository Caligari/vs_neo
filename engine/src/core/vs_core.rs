use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use common::SharedTimeVal;

use crate::core::core_game::*;
use crate::core::core_game_registry::{CoreGameRegistry, GameId};
use crate::core::game_systems::GameSystems;
use crate::timer_system::TimerSystem;
// use crate::utils::preferences::Preferences;

pub type GameSystemMap = Vec<Rc<RefCell<GameSystem>>>; // , CoreGameSystem>;

pub struct Core {
    // !! game_heap: VSheap,  // trying not to need this
    pub game_registry: CoreGameRegistry,
    pub game_systems: GameSystems,

    current_game: Option<GameId>,
    next_game: Option<GameId>,

    // preferences: Preferences,
    temp_refresh_rate: Arc<Mutex<u16>>,
    launch_time: SharedTimeVal,

    exit: bool,
    allow_exit: bool,
}

impl Core {
    pub fn new(launch_time: SharedTimeVal) -> Self {
        Core {
            game_registry: CoreGameRegistry::default(),
            game_systems: GameSystems::new(),
            current_game: None,
            next_game: None,
            // preferences: Preferences::new(save_file),
            exit: false,
            allow_exit: true,
            temp_refresh_rate: Arc::new(Mutex::new(60)),
            launch_time,
        }
    }
    pub fn init(&mut self) {
        self.create_game_systems(); // moved here from start of Go
    }

    pub fn deinit(&mut self) {}

    fn create_game_systems(&mut self) {
        // let timer = new_RefTimerSystem(self.temp_refresh_rate.clone(), self.launch_time.clone());
        self.game_systems.set_timer(Box::new(TimerSystem::new(
            self.temp_refresh_rate.clone(),
            self.launch_time.clone(),
        )));
        // .push(Rc::new(RefCell::new(GameSystem::Timer(timer))));
        // TODO: create these systems
        // self.game_systems.insert(GameSystem::Timer, TimerSystem::new());
        // self.game_systems.insert(GameSystem::Input, InputSystem::new());
        // self.game_systems.insert(GameSystem::Collision, CollisionSystem::new());
        // self.game_systems.insert(GameSystem::Sound, SoundSystem::new());
    }

    pub fn register_game(
        &mut self,
        name: &str,
        main_game: bool,
        code: Box<dyn GameCode>,
    ) -> GameId {
        let entry = CoreGame::new(name, code, &mut self.game_systems);
        self.game_registry.register_game(entry, main_game)
    }

    // pub fn get_game_system ( &mut self, system: GameSystem ) -> Option<&mut CoreGameSystem> {
    //     if let Some(sys) = self.game_systems.get_mut(&system) {
    //         Some(sys)
    //     } else { None }
    // }

    /// Marks a different game as becoming active.  The transition to the new game won't happen immediately,
    /// but will be delayed until the next frame's update cycle.  This is to protect against any weirdnesses
    /// which could be caused by a game transition occurring during the middle of an update or drawing operation.
    pub fn set_game(&mut self, game: GameId) {
        self.next_game = Some(game);
    }

    /// This is our high-level game-running logic.  It brings new games in and out, as requested.
    /// It also initiates memory validation checks between games, and prints out game statistics at that time.
    pub fn go(&mut self) {
        // self.create_game_systems();  // now in init(), before any game is created

        while !self.exit || !self.allow_exit {
            if let Some(new_game) = self.next_game {
                if let Some(current_game) = self.current_game {
                    let game = self
                        .game_registry
                        .get_game(current_game)
                        .expect("unable to find current game, when preparing for next game");
                    game.stop_timer();
                    game.deinit();
                }

                self.current_game = Some(new_game);
                let game = self
                    .game_registry
                    .get_game(new_game)
                    .expect("unable to find next game, when switching to next game");
                self.next_game = None;
                game.init();
                game.start_timer();
            }

            if let Some(current_game) = self.current_game {
                let game = self
                    .game_registry
                    .get_game(current_game)
                    .expect("unable to find next game, when switching to next game");
                game.go(&mut self.game_systems);
                self.exit = game.should_exit();
            }
        }
    }
}
