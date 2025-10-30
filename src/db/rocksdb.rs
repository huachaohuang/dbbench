use std::hint::black_box;

use anyhow::Result;
use rocksdb::{
    BlockBasedOptions, Cache, DB, DBCompressionType, IteratorMode, Options as DbOptions,
};

use crate::db::{Database, Options};

pub struct Rocksdb {
    db: DB,
}

impl Rocksdb {
    pub fn open(options: Options) -> Result<Self> {
        let block_cache_size = options.cache_size / 8 * 7;
        let write_buffer_size = options.cache_size - block_cache_size;
        let num_background_threads = 4;
        let mut topts = BlockBasedOptions::default();
        topts.set_block_cache(&Cache::new_lru_cache(block_cache_size));
        topts.set_bloom_filter(10.0, true);
        topts.set_cache_index_and_filter_blocks(true);
        let mut dbopts = DbOptions::default();
        dbopts.create_if_missing(true);
        dbopts.set_enable_pipelined_write(true);
        dbopts.set_optimize_filters_for_hits(true);
        dbopts.set_avoid_unnecessary_blocking_io(true);
        dbopts.set_compression_type(DBCompressionType::None);
        dbopts.increase_parallelism(num_background_threads);
        dbopts.set_block_based_table_factory(&topts);
        dbopts.optimize_level_style_compaction(write_buffer_size);
        let db = DB::open(&dbopts, options.path)?;
        Ok(Self { db })
    }
}

impl Database for Rocksdb {
    fn read(&self, k: &[u8]) -> Result<()> {
        black_box({
            self.db.get(k)?;
        });
        Ok(())
    }

    fn scan(&self, k: &[u8], n: usize) -> Result<()> {
        let mut iter = self
            .db
            .iterator(IteratorMode::From(k, rocksdb::Direction::Forward));
        black_box({
            for _ in 0..n {
                iter.next();
            }
        });
        Ok(())
    }

    fn write(&self, k: &[u8], v: &[u8]) -> Result<()> {
        self.db.put(k, v)?;
        Ok(())
    }
}
