use clap::Args;
use rand::{Rng, distr::weighted::WeightedIndex, rng};

#[derive(Clone, Debug, Args)]
#[group(skip)]
pub struct Options {
    #[arg(long, short, default_value_t = 0.95)]
    pub read_ratio: f32,
    #[arg(long, short, default_value_t = 0.00)]
    pub scan_ratio: f32,
    #[arg(long, short, default_value_t = 0.05)]
    pub write_ratio: f32,
}

impl Options {
    pub fn new_for_load() -> Self {
        Self {
            read_ratio: 0.0,
            scan_ratio: 0.0,
            write_ratio: 1.0,
        }
    }
}

pub struct Workload {
    dist: WeightedIndex<f32>,
}

impl Workload {
    pub fn new(options: Options) -> Self {
        let dist =
            WeightedIndex::new([options.read_ratio, options.scan_ratio, options.write_ratio])
                .unwrap();
        Self { dist }
    }

    pub fn next(&self) -> Operation {
        rng().sample(&self.dist).into()
    }
}

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum Operation {
    Read = 0,
    Scan = 1,
    Write = 2,
}

impl Operation {
    pub const COUNT: usize = 3;
}

impl From<usize> for Operation {
    fn from(value: usize) -> Self {
        match value {
            0 => Operation::Read,
            1 => Operation::Scan,
            2 => Operation::Write,
            _ => unreachable!(),
        }
    }
}
