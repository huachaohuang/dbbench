use anyhow::Result;
use clap::{Parser, Subcommand};

mod dataset;
mod db;
mod generator;
mod load;
mod open;
mod run;
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
    Run(run::Command),
    /// Load data into a database
    Load(load::Command),
    /// Open a database for inspection
    Open(open::Command),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Run(cmd) => cmd.run(),
        Command::Load(cmd) => cmd.run(),
        Command::Open(cmd) => cmd.run(),
    }
}
