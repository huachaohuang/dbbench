use std::{hint::black_box, ops::Bound};

use anyhow::Result;
use heed::{Env, EnvFlags, EnvOpenOptions, types::Bytes};

use crate::db::{Database, Options};

pub struct Lmdb {
    env: Env,
    db: heed::Database<Bytes, Bytes>,
}

impl Lmdb {
    pub fn open(options: Options) -> Result<Self> {
        let env = unsafe {
            let mut builder = EnvOpenOptions::new();
            if !options.sync {
                builder.flags(EnvFlags::NO_SYNC);
            }
            builder.map_size(options.cache_size);
            builder.open(&options.path)?
        };
        let mut txn = env.write_txn()?;
        let db = env.create_database(&mut txn, None)?;
        txn.commit()?;
        Ok(Self { env, db })
    }
}

impl Database for Lmdb {
    fn read(&self, k: &[u8]) -> Result<()> {
        let txn = self.env.read_txn()?;
        black_box({
            self.db.get(&txn, k)?;
        });
        txn.commit()?;
        Ok(())
    }

    fn scan(&self, k: &[u8], n: usize) -> Result<()> {
        let txn = self.env.read_txn()?;
        let range = (Bound::Included(k), Bound::Unbounded);
        black_box({
            let mut iter = self.db.range(&txn, &range)?;
            for _ in 0..n {
                iter.next();
            }
        });
        txn.commit()?;
        Ok(())
    }

    fn write(&self, k: &[u8], v: &[u8]) -> Result<()> {
        let mut txn = self.env.write_txn()?;
        self.db.put(&mut txn, k, v)?;
        txn.commit()?;
        Ok(())
    }
}
