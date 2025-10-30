use clap::{Args, ValueEnum};
use rand::{Rng, rng};

use crate::generator::{Generator, SequentialGenerator, UniformGenerator, ZipfianGenerator};

#[derive(Args, Clone, Debug)]
#[group(skip)]
pub struct Options {
    #[arg(long, short, default_value_t = 10)]
    klen: usize,
    #[arg(long, short, default_value_t = 100)]
    vlen: usize,
    #[arg(long, short, default_value_t = 1000)]
    num_records: usize,
    #[arg(long, short, value_enum, default_value_t = Distribution::Uniform)]
    distribution: Distribution,
}

pub struct Dataset {
    options: Options,
    generator: Box<dyn Generator>,
}

impl Dataset {
    pub fn new(options: Options) -> Self {
        let generator = options.distribution.new_generator();
        Self { options, generator }
    }
}

impl Dataset {
    pub fn next(&self, k: &mut Vec<u8>) {
        let x = self.generator.next() % self.options.num_records as u64;
        let b = x.to_be_bytes();
        k.clear();
        if let Some(i) = b.len().checked_sub(self.options.klen) {
            k.extend_from_slice(&b[i..]);
        } else {
            k.extend_from_slice(&b);
            k.resize(self.options.klen, 0);
        }
    }

    pub fn next_record(&self, k: &mut Vec<u8>, v: &mut Vec<u8>) {
        self.next(k);
        v.resize(self.options.vlen, 0);
        rng().fill(&mut v[..]);
    }
}

#[derive(Clone, Debug, ValueEnum)]
enum Distribution {
    Uniform,
    Zipfian,
    Sequential,
}

impl Distribution {
    fn new_generator(&self) -> Box<dyn Generator> {
        match self {
            Self::Uniform => Box::new(UniformGenerator::new()),
            Self::Zipfian => Box::new(ZipfianGenerator::new()),
            Self::Sequential => Box::new(SequentialGenerator::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    const NUM_RECORDS: usize = 100;

    fn test_dataset(options: Options) {
        let dataset = Dataset::new(options.clone());
        let mut k = Vec::new();
        let mut count = BTreeMap::new();
        for _ in 0..(options.num_records * 10) {
            dataset.next(&mut k);
            count.entry(k.clone()).and_modify(|c| *c += 1).or_insert(1);
        }
        println!("{count:#?}");
    }

    #[test]
    #[ignore]
    fn test_uniform_dataset() {
        let options = Options {
            klen: 4,
            vlen: 100,
            num_records: NUM_RECORDS,
            distribution: Distribution::Uniform,
        };
        test_dataset(options);
    }

    #[test]
    #[ignore]
    fn test_zipfian_dataset() {
        let options = Options {
            klen: 8,
            vlen: 100,
            num_records: NUM_RECORDS,
            distribution: Distribution::Zipfian,
        };
        test_dataset(options);
    }

    #[test]
    #[ignore]
    fn test_sequential_dataset() {
        let options = Options {
            klen: 10,
            vlen: 100,
            num_records: NUM_RECORDS,
            distribution: Distribution::Sequential,
        };
        test_dataset(options);
    }
}
