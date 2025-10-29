use rand::{Rng, rng};
use rand_distr::StandardUniform;

use crate::generator::Generator;

pub struct Uniform {
    dist: StandardUniform,
}

impl Uniform {
    pub fn new() -> Self {
        Self {
            dist: StandardUniform,
        }
    }
}

impl Generator for Uniform {
    fn next(&self) -> u64 {
        rng().sample(&self.dist)
    }
}
