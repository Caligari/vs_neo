use std::{cell::RefCell, rc::Rc};

use crate::collision_system::CollisionSystem;

pub mod collision_system;

pub type RefCollisionSystem = Rc<RefCell<CollisionSystem>>;

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
