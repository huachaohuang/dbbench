use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct Command {
    #[command(flatten)]
    db: crate::db::Options,
}

impl Command {
    pub fn run(self) -> Result<()> {
        todo!()
    }
}
