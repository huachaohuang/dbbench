use anyhow::Result;
use clap::{Args, ValueEnum};

pub trait Db {
    fn read(&self, key: &[u8]) -> Result<()>;

    fn write(&self, key: &[u8], value: &[u8]) -> Result<()>;
}

#[derive(Clone, ValueEnum)]
pub enum Name {
    Lmdb,
    Rocksdb,
    Wiredtiger,
}

#[derive(Args)]
pub struct Options {
    name: Name,
    path: String,
}

impl Options {
    pub fn open(self) -> Result<Box<dyn Db>> {
        todo!()
    }
}
