pub mod color;
pub mod math;
pub mod utils;

use std::sync::Arc;
use std::sync::Mutex;

pub use crate::color::*;
pub use crate::math::spline::*;
pub use crate::math::spring::*;
pub use crate::math::vector::*;

pub type SharedTimeVal = Arc<Mutex<Option<u64>>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
