use anyhow::Result;
use clap::{Args, ValueEnum};

#[derive(Args)]
pub struct Command {
    #[command(flatten)]
    db: crate::db::Options,
    #[command(flatten)]
    options: crate::Options,
    #[arg(long, short, value_enum, default_value_t = Distribution::Sequential)]
    distribution: Distribution,
}

#[derive(Clone, ValueEnum)]
enum Distribution {
    Uniform,
    Sequential,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        todo!()
    }
}
