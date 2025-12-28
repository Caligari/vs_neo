use turborand::prelude::*;

#[allow(dead_code)]
pub struct Random {
    rand: Rng,
}

impl Random {
    pub fn new() -> Random {
        Random { rand: Rng::new() }
    }

    pub fn new_from_seed(seed: u32) -> Random {
        Random {
            rand: Rng::with_seed(seed as u64),
        }
    }

    pub fn new_from_string(seed_string: &str) -> Random {
        let seed = seed_string.bytes().fold(0, |acc, b| acc + b as u32);
        Random::new_from_seed(seed)
    }
}
