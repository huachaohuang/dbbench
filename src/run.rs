use anyhow::Result;
use clap::{Args, ValueEnum};

#[derive(Args)]
pub struct Command {
    #[command(flatten)]
    db: crate::db::Options,
    #[command(flatten)]
    options: crate::Options,
    #[arg(long, short, default_value_t = 1000)]
    operations: usize,
    #[arg(long, short, default_value_t = 0.95)]
    read_ratio: f32,
    #[arg(long, short, default_value_t = 0.05)]
    write_ratio: f32,
    #[arg(long, short, value_enum, default_value_t = Distribution::Uniform)]
    distribution: Distribution,
}

#[derive(Clone, ValueEnum)]
enum Distribution {
    Uniform,
    Zipfian,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        todo!()
    }
}
