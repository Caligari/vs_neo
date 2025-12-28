use crate::core::core_game::CoreGame;

pub trait CoreGameSystem {
    fn is_active(&self) -> bool;
    fn init(&mut self);
    fn deinit(&mut self);
    fn update(&mut self, core: &mut CoreGame);
    fn post_update(&mut self, core: &mut CoreGame);
    fn set_active(&mut self, active: bool);
}
