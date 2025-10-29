use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct Command {
    #[command(flatten)]
    db: crate::db::Options,
    #[command(flatten)]
    dataset: crate::dataset::Options,
    #[command(flatten)]
    workload: crate::workload::Options,
    #[arg(long, short = 't', default_value_t = 1)]
    num_threads: usize,
    #[arg(long, short = 'o', default_value_t = 1000)]
    num_operations: usize,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        todo!()
    }
}
