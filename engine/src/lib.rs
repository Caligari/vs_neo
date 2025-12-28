use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use common::SharedTimeVal;

use crate::{core::core_game::GameSystem, timer_system::TimerSystem};

pub mod core;
pub mod system;
pub mod system_preferences;
pub mod timer_system;

pub type RefGameSystem = Rc<RefCell<GameSystem>>;
pub type RefTimerSystem = Rc<RefCell<TimerSystem>>;

#[allow(non_snake_case)]
pub fn new_RefTimerSystem(
    refresh_rate: Arc<Mutex<u16>>,
    launch_time: SharedTimeVal,
) -> RefTimerSystem {
    Rc::new(RefCell::new(TimerSystem::new(refresh_rate, launch_time)))
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
