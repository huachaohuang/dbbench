use anyhow::Result;
use clap::{Args, ValueEnum};

mod lmdb;
use lmdb::Lmdb;

mod rocksdb;
use rocksdb::Rocksdb;

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
    #[arg(long, default_value_t = false)]
    pub sync: bool,
    #[arg(long, default_value_t = 128 * 1024 * 1024)]
    pub cache_size: usize,
}

impl Options {
    pub fn open(self) -> Result<Box<dyn Database>> {
        std::fs::create_dir_all(&self.path)?;
        match self.db {
            Name::Lmdb => {
                let db = Lmdb::open(self)?;
                Ok(Box::new(db))
            }
            Name::Rocksdb => {
                let db = Rocksdb::open(self)?;
                Ok(Box::new(db))
            }
            Name::Wiredtiger => todo!(),
        }
    }
}

pub trait Database: Send + Sync {
    fn stat(&self) -> Result<String>;

    fn read(&self, k: &[u8]) -> Result<bool>;

    fn scan(&self, k: &[u8], n: usize) -> Result<()>;

    fn write(&self, k: &[u8], v: &[u8]) -> Result<()>;
}
