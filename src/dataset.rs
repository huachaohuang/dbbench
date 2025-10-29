use clap::{Args, ValueEnum};
use rand::{Rng, rng};

use crate::generator::{Generator, Sequential, Uniform, Zipfian};

#[derive(Args)]
#[group(skip)]
pub struct Options {
    #[arg(long, short = 'k', default_value_t = 10)]
    klen: usize,
    #[arg(long, short = 'v', default_value_t = 100)]
    vlen: usize,
    #[arg(long, short = 'n', default_value_t = 1000)]
    records: usize,
    #[arg(long, short = 'd', value_enum, default_value_t = Distribution::Uniform)]
    distribution: Distribution,
}

#[derive(Clone, ValueEnum)]
enum Distribution {
    Uniform,
    Zipfian,
    Sequential,
}

pub struct Dataset {
    options: Options,
    generator: Box<dyn Generator>,
}

impl Dataset {
    pub fn new(options: Options) -> Self {
        let generator: Box<dyn Generator> = match options.distribution {
            Distribution::Uniform => Box::new(Uniform::new()),
            Distribution::Zipfian => Box::new(Zipfian::new(options.records as u64)),
            Distribution::Sequential => Box::new(Sequential::new()),
        };
        Self { options, generator }
    }
}

impl Dataset {
    pub fn next(&self, k: &mut Vec<u8>) {
        let x = self.generator.next() % self.options.records as u64;
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
