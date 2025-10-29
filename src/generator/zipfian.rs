use std::hash::BuildHasher;

use fnv::FnvBuildHasher;
use rand::{Rng, rng};
use rand_distr::Zipf;

use crate::generator::Generator;

#[derive(Clone)]
pub struct Zipfian {
    dist: Zipf<f64>,
    hash: FnvBuildHasher,
}

impl Zipfian {
    pub fn new(n: u64) -> Self {
        Self {
            dist: Zipf::new(n as f64, 0.99).unwrap(),
            hash: FnvBuildHasher::new(),
        }
    }
}

impl Generator for Zipfian {
    fn next(&self) -> u64 {
        let x = rng().sample(&self.dist) as u64;
        self.hash.hash_one(x)
    }
}
