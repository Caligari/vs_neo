use std::{cell::RefCell, rc::Rc};

use crate::sound_system::SoundSystem;

pub mod sound_system;

pub type RefSoundSystem = Rc<RefCell<SoundSystem>>;

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
