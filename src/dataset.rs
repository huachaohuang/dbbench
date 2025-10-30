use clap::{Args, ValueEnum};
use rand::{Rng, rng};

use crate::generator::{Generator, SequentialGenerator, UniformGenerator, ZipfianGenerator};

#[derive(Args, Debug)]
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
