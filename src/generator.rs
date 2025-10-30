use std::{
    hash::BuildHasher,
    sync::atomic::{AtomicU64, Ordering},
};

use fnv::FnvBuildHasher;
use rand::{Rng, rng};
use rand_distr::{StandardUniform, Zipf};

pub trait Generator: Send + Sync {
    fn next(&self) -> u64;
}

pub struct UniformGenerator {
    dist: StandardUniform,
}

impl UniformGenerator {
    pub fn new() -> Self {
        Self {
            dist: StandardUniform,
        }
    }
}

impl Generator for UniformGenerator {
    fn next(&self) -> u64 {
        rng().sample(&self.dist)
    }
}

pub struct ZipfianGenerator {
    dist: Zipf<f64>,
    hash: FnvBuildHasher,
}

impl ZipfianGenerator {
    // Constants from YCSB
    const N: f64 = 10_000_000_000.0;
    const S: f64 = 0.99;

    pub fn new() -> Self {
        Self {
            dist: Zipf::new(Self::N, Self::S).unwrap(),
            hash: FnvBuildHasher::new(),
        }
    }
}

impl Generator for ZipfianGenerator {
    fn next(&self) -> u64 {
        let x = rng().sample(&self.dist) as u64;
        // Scatter hotspots
        self.hash.hash_one(x)
    }
}

pub struct SequentialGenerator {
    count: AtomicU64,
}

impl SequentialGenerator {
    pub fn new() -> Self {
        Self {
            count: AtomicU64::new(0),
        }
    }
}

impl Generator for SequentialGenerator {
    fn next(&self) -> u64 {
        self.count.fetch_add(1, Ordering::Relaxed)
    }
}
