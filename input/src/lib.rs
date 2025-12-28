use std::{cell::RefCell, rc::Rc};

use crate::input_system::InputSystem;

pub mod input_system;

pub type RefInputSystem = Rc<RefCell<InputSystem>>;

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
