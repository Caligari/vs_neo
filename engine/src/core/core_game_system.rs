use downcast_rs::{Downcast, impl_downcast};

use crate::core::core_game::CoreGame;

pub trait CoreGameSystem: Downcast {
    fn is_active(&self) -> bool;
    fn init(&mut self);
    fn deinit(&mut self);
    fn update(&mut self, core: &mut CoreGame);
    fn post_update(&mut self, core: &mut CoreGame);
    fn set_active(&mut self, active: bool);
}
impl_downcast!(CoreGameSystem);
