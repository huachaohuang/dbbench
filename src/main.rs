use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod db;
mod generator;

mod runtime;
use runtime::Runtime;

mod dataset;
use dataset::Dataset;

mod workload;
use workload::Workload;

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

#[derive(Args, Debug)]
struct RunCommand {
    #[command(flatten)]
    db: db::Options,
    #[command(flatten)]
    dataset: dataset::Options,
    #[command(flatten)]
    workload: workload::Options,
    #[arg(long, short = 't', default_value_t = 1)]
    num_threads: usize,
    #[arg(long, short = 'o', default_value_t = 10000)]
    num_operations: usize,
}

impl RunCommand {
    fn run(self) -> Result<()> {
        let db = self.db.open()?;
        let dataset = Dataset::new(self.dataset);
        let workload = Workload::new(self.workload);
        Runtime::new(db, dataset, workload).run(self.num_threads, self.num_operations)
    }
}

#[derive(Args, Debug)]
struct LoadCommand {
    #[command(flatten)]
    db: db::Options,
    #[command(flatten)]
    dataset: dataset::Options,
    #[arg(long, short = 't', default_value_t = 1)]
    num_threads: usize,
}

impl LoadCommand {
    fn run(self) -> Result<()> {
        let num_operations = self.dataset.num_records;
        let cmd = RunCommand {
            db: self.db,
            dataset: self.dataset,
            workload: workload::Options::new_for_load(),
            num_threads: self.num_threads,
            num_operations,
        };
        cmd.run()
    }
}

#[derive(Args, Debug)]
struct OpenCommand {
    #[command(flatten)]
    db: db::Options,
}

impl OpenCommand {
    fn run(self) -> Result<()> {
        todo!()
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Run(cmd) => {
            println!("{cmd:#?}");
            cmd.run()
        }
        Command::Load(cmd) => {
            println!("{cmd:#?}");
            cmd.run()
        }
        Command::Open(cmd) => {
            println!("{cmd:#?}");
            cmd.run()
        }
    }
}
