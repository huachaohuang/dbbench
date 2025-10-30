use anyhow::Result;
use clap::{Args, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
pub enum Name {
    Lmdb,
    Rocksdb,
    Wiredtiger,
}

#[derive(Clone, Debug, Args)]
pub struct Options {
    pub db: Name,
    pub path: String,
}

impl Options {
    pub fn open(self) -> Result<Box<dyn Database>> {
        todo!()
    }
}

pub trait Database: Send + Sync {
    fn read(&self, k: &[u8]) -> Result<()>;

    fn scan(&self, k: &[u8], n: usize) -> Result<()>;

    fn write(&self, k: &[u8], v: &[u8]) -> Result<()>;
}
