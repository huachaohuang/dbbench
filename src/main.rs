use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod db;
mod generator;
mod load;
mod open;
mod run;

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a benchmark
    Run(run::Command),
    /// Load data into a database
    Load(load::Command),
    /// Open a database for inspection
    Open(open::Command),
}

#[derive(Args)]
#[group(skip)]
struct Options {
    #[arg(long, short = 'k', default_value_t = 10)]
    klen: usize,
    #[arg(long, short = 'v', default_value_t = 100)]
    vlen: usize,
    #[arg(long, short = 'n', default_value_t = 1000)]
    records: usize,
    #[arg(long, short = 't', default_value_t = 1)]
    threads: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Run(cmd) => cmd.run(),
        Command::Load(cmd) => cmd.run(),
        Command::Open(cmd) => cmd.run(),
    }
}
