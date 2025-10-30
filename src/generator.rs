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
    pub fn new() -> Self {
        Self {
            dist: Zipf::new(u64::MAX as f64, 0.99).unwrap(),
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    const MAX: u64 = 100;

    fn test_generator<G: Generator>(generator: G) {
        let mut count = BTreeMap::new();
        for _ in 0..(MAX * 10) {
            let x = generator.next() % MAX;
            count.entry(x).and_modify(|c| *c += 1).or_insert(1);
        }
        println!("{:#?}", count);
    }

    #[test]
    #[ignore]
    fn test_uniform_generator() {
        test_generator(UniformGenerator::new());
    }

    #[test]
    #[ignore]
    fn test_zipfian_generator() {
        test_generator(ZipfianGenerator::new());
    }

    #[test]
    #[ignore]
    fn test_sequential_generator() {
        test_generator(SequentialGenerator::new());
    }
}
