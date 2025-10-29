use clap::Args;
use rand::{Rng, distr::weighted::WeightedIndex, rng};

#[derive(Args)]
pub struct Options {
    #[arg(long, short, default_value_t = 0.95)]
    read_ratio: f32,
    #[arg(long, short, default_value_t = 0.00)]
    scan_ratio: f32,
    #[arg(long, short, default_value_t = 0.05)]
    write_ratio: f32,
}

pub struct Workload {
    dist: WeightedIndex<f32>,
}

impl Workload {
    pub fn new(options: Options) -> Self {
        let dist =
            WeightedIndex::new(&[options.read_ratio, options.scan_ratio, options.write_ratio])
                .unwrap();
        Self { dist }
    }

    pub fn next(&self) -> Operation {
        match rng().sample(&self.dist) {
            0 => Operation::Read,
            1 => Operation::Scan,
            2 => Operation::Write,
            _ => unreachable!(),
        }
    }
}

pub enum Operation {
    Read,
    Scan,
    Write,
}
