use elsa::FrozenIndexMap;

use common::{GameSystemType, vs_error::VSError};

use crate::{
    core::{core_game::CoreGame, core_game_system::CoreGameSystem},
    timer_system::TimerSystem,
};

pub struct GameSystems {
    systems: FrozenIndexMap<GameSystemType, Box<dyn CoreGameSystem>>,
    systems_order: Vec<GameSystemType>,
    timer: Option<Box<TimerSystem>>, // reference with lifetime?
}

impl GameSystems {
    pub fn new() -> Self {
        GameSystems {
            systems: FrozenIndexMap::new(),
            systems_order: Vec::new(),
            timer: None,
        }
    }

    pub fn set_timer(&mut self, timer: Box<TimerSystem>) -> Result<(), VSError> {
        if self.timer.is_none() {
            self.timer = Some(timer);
            self.systems_order.push(GameSystemType::Timer);
            Ok(())
        } else {
            Err(VSError::Core_DuplicateGameSystem)
        }
    }

    pub fn insert(
        &mut self,
        system_type: GameSystemType,
        system: Box<dyn CoreGameSystem>,
    ) -> Result<(), VSError> {
        // is bool right?
        if self.systems.get_index_of(&system_type).is_none() {
            self.systems.insert(system_type, system);
            Ok(())
        } else {
            Err(VSError::Core_DuplicateGameSystem)
        }
    }

    pub fn update_system(
        &mut self,
        system_type: GameSystemType,
        game: &mut CoreGame,
        pre_post: PrePostUpdate,
    ) -> Result<(), VSError> {
        match system_type {
            GameSystemType::Timer => {
                if let Some(system_boxed) = self.timer.as_mut() {
                    if system_boxed.is_active() {
                        if pre_post == PrePostUpdate::PreUpdate {
                            Ok(system_boxed.update(game))
                        } else {
                            Ok(system_boxed.post_update(game))
                        }
                    } else {
                        Ok(())
                    }
                } else {
                    Err(VSError::Core_SystemNotFound(system_type))
                }
            }
            // GameSystemType::Collision => ,
            // GameSystemType::Input => ,
            // GameSystemType::Sound => ,
            _ => {
                if let Some(system_boxed) = self.systems.as_mut().get_mut(&system_type) {
                    if system_boxed.is_active() {
                        if pre_post == PrePostUpdate::PreUpdate {
                            Ok(system_boxed.update(game))
                        } else {
                            Ok(system_boxed.post_update(game))
                        }
                    } else {
                        Ok(())
                    }
                } else {
                    Err(VSError::Core_SystemNotFound(system_type))
                }
            }
        }
    }

    pub fn get_timer(&mut self) -> Option<&TimerSystem> {
        self.timer.as_deref()
    }

    pub fn system_order(&self) -> Vec<GameSystemType> {
        self.systems_order.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrePostUpdate {
    PreUpdate,
    PostUpdate,
}
