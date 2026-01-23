use elsa::FrozenIndexMap;

use common::{GameSystemType, vs_error::VSError};

use crate::{
    core::{core_game::CoreGame, core_game_system::CoreGameSystem},
    timer_system::TimerSystem,
};

pub struct GameSystems {
    systems: FrozenIndexMap<GameSystemType, Box<dyn CoreGameSystem>>,
    systems_order: Vec<GameSystemType>,
    timer: Option<Box<dyn CoreGameSystem>>, // reference with lifetime?
}

impl Default for GameSystems {
    fn default() -> Self {
        GameSystems {
            systems: FrozenIndexMap::new(),
            systems_order: Vec::new(),
            timer: None,
        }
    }
}

impl GameSystems {
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
        let Some(system) = (match system_type {
            GameSystemType::Timer => self.timer.as_mut(),
            // GameSystemType::Collision => ,
            // GameSystemType::Input => ,
            // GameSystemType::Sound => ,
            _ => self.systems.as_mut().get_mut(&system_type),
        }) else {
            return Err(VSError::Core_SystemNotFound(system_type));
        };

        if system.is_active() {
            match pre_post {
                PrePostUpdate::PreUpdate => {
                    system.update(game);
                    Ok(())
                }
                PrePostUpdate::PostUpdate => {
                    system.post_update(game);
                    Ok(())
                }
            }
        } else {
            Ok(())
        }
    }

    pub fn get_timer(&mut self) -> Option<&mut TimerSystem> {
        if self.timer.is_some() {
            self.timer.as_mut().unwrap().downcast_mut::<TimerSystem>()
        } else {
            None
        }
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
