use rand::{Rng, rng};
use rand_distr::StandardUniform;

use crate::generator::Generator;

pub struct UniformGenerator {
    dist: StandardUniform,
}

impl Generator for UniformGenerator {
    fn next(&self) -> u64 {
        rng().sample(&self.dist)
    }
}
