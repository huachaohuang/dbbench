use anyhow::Result;
use clap::{Args, ValueEnum};

pub trait Db: Send + Sync {
    fn read(&self, k: &[u8]) -> Result<()>;

    fn scan(&self, k: &[u8], n: usize) -> Result<()>;

    fn write(&self, k: &[u8], v: &[u8]) -> Result<()>;
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Name {
    Lmdb,
    Rocksdb,
    Wiredtiger,
}

#[derive(Args, Debug)]
pub struct Options {
    db: Name,
    path: String,
}

impl Options {
    pub fn open(self) -> Result<Box<dyn Db>> {
        todo!()
    }
}
