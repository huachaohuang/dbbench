use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod dataset;
mod db;
mod generator;
mod runtime;
mod workload;

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a benchmark
    Run(RunCommand),
    /// Load data into a database
    Load(LoadCommand),
    /// Open a database for inspection
    Open(OpenCommand),
}

#[derive(Args)]
struct RunCommand {
    #[command(flatten)]
    db: db::Options,
    #[command(flatten)]
    dataset: dataset::Options,
    #[command(flatten)]
    workload: workload::Options,
    #[arg(long, short = 't', default_value_t = 1)]
    num_threads: usize,
    #[arg(long, short = 'o', default_value_t = 1000)]
    num_operations: usize,
}

impl RunCommand {
    fn run(&self) -> Result<()> {
        todo!()
    }
}

#[derive(Args)]
struct LoadCommand {
    #[command(flatten)]
    db: db::Options,
    #[command(flatten)]
    dataset: dataset::Options,
    #[arg(long, short = 't', default_value_t = 1)]
    num_threads: usize,
}

impl LoadCommand {
    fn run(&self) -> Result<()> {
        todo!()
    }
}

#[derive(Args)]
struct OpenCommand {
    #[command(flatten)]
    db: crate::db::Options,
}

impl OpenCommand {
    fn run(self) -> Result<()> {
        todo!()
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Run(cmd) => cmd.run(),
        Command::Load(cmd) => cmd.run(),
        Command::Open(cmd) => cmd.run(),
    }
}
