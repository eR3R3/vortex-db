use std::sync::Arc;
use std::thread;
use std::time::Duration;
use rocksdb::{BlockBasedOptions, Cache, DBCompactionStyle, DBCompressionType, OptimisticTransactionDB, OptimisticTransactionOptions, Options, ReadOptions, WriteOptions};
use anyhow::Result;
use crate::storage::rocksdb::transaction::{Check, Transaction};
use super::super::super::config;

pub struct Datastore {
    db: Arc<OptimisticTransactionDB>,
}

impl Datastore {
    pub(crate) fn new(path: &str) -> Result<Datastore> {
        // Configure custom options
        let mut opts = Options::default();
        // Ensure we use fdatasync
        opts.set_use_fsync(false);
        // Create database if missing
        opts.create_if_missing(true);
        // Create column families if missing
        opts.create_missing_column_families(true);
        // Increase the background thread count
        opts.increase_parallelism(*config::ROCKSDB_THREAD_COUNT);
        opts.set_max_background_jobs(*config::ROCKSDB_JOBS_COUNT);
        opts.set_max_open_files(config::ROCKSDB_MAX_OPEN_FILES);

        // WAL
        opts.set_keep_log_file_num(config::ROCKSDB_KEEP_LOG_FILE_NUM);
        opts.set_wal_size_limit_mb(config::ROCKSDB_WAL_SIZE_LIMIT);

        // MEMTABLE
        opts.set_max_write_buffer_number(config::ROCKSDB_MAX_WRITE_BUFFER_NUMBER);
        opts.set_write_buffer_size(config::ROCKSDB_WRITE_BUFFER_SIZE);
        opts.set_min_write_buffer_number_to_merge(config::ROCKSDB_MIN_WRITE_BUFFER_NUMBER_TO_MERGE);
        opts.set_allow_concurrent_memtable_write(true);

        // SSTABLE
        opts.set_target_file_size_base(config::ROCKSDB_TARGET_FILE_SIZE_BASE);
        opts.set_target_file_size_multiplier(config::ROCKSDB_TARGET_FILE_SIZE_MULTIPLIER);
        opts.set_level_zero_file_num_compaction_trigger(config::ROCKSDB_FILE_COMPACTION_TRIGGER);
        opts.set_max_subcompactions(config::ROCKSDB_MAX_CONCURRENT_SUBCOMPACTIONS);

        // BLOB
        opts.set_enable_blob_files(true);
        opts.set_min_blob_size(config::ROCKSDB_MIN_BLOB_SIZE);

        // OTHER
        opts.set_enable_pipelined_write(true);
        opts.set_avoid_unnecessary_blocking_io(true);
        opts.set_enable_write_thread_adaptive_yield(true);

        // CACHE
        let cache = Cache::new_lru_cache(config::ROCKSDB_BLOCK_CACHE_SIZE);

        let mut block_opts = BlockBasedOptions::default();
        block_opts.set_pin_l0_filter_and_index_blocks_in_cache(true);
        block_opts.set_pin_top_level_index_and_filter(true);
        block_opts.set_bloom_filter(10.0, false);
        block_opts.set_block_size(config::ROCKSDB_BLOCK_SIZE);
        block_opts.set_block_cache(&cache);

        // Configure the database with the cache
        opts.set_block_based_table_factory(&block_opts);
        opts.set_blob_cache(&cache);
        opts.set_row_cache(&cache);

        // MMAP
        opts.set_allow_mmap_reads(config::ROCKSDB_ENABLE_MEMORY_MAPPED_READS);
        opts.set_allow_mmap_writes(false);

        // COMPACTION
        opts.set_compaction_style(DBCompactionStyle::Level);
        opts.set_compression_per_level(&[
            DBCompressionType::None,
            DBCompressionType::None,
            DBCompressionType::Snappy,
            DBCompressionType::Snappy,
            DBCompressionType::Snappy,
        ]);

        let db = match config::ROCKSDB_BACKGROUND_FLUSH {
            false => {
                // do not enable manual WAL flush
                opts.set_manual_wal_flush(false);
                // Create the optimistic datastore
                Arc::new(OptimisticTransactionDB::open(&opts, path)?)
            }
            true => {
                opts.set_manual_wal_flush(true);
                let db = Arc::new(OptimisticTransactionDB::open(&opts, path)?);
                let dbc = db.clone();
                thread::spawn(move || loop {
                    // Get the specified flush interval
                    let wait = config::ROCKSDB_BACKGROUND_FLUSH_INTERVAL;
                    // Wait for the specified interval
                    thread::sleep(Duration::from_millis(wait));
                    // Flush the WAL to disk periodically
                    if let Err(err) = dbc.flush_wal(config::SYNC_DATA) {
                        println!("Failed to flush WAL: {err}");
                    }
                });
                db
            }
        };

        Ok(Datastore {
            db,
        })
    }

    pub(crate) fn transaction(&self, write: bool, _: bool) -> Result<Transaction> {
        // Set the transaction options
        let mut to = OptimisticTransactionOptions::default();
        to.set_snapshot(true);
        // Set the write options
        let mut wo = WriteOptions::default();
        wo.set_sync(config::SYNC_DATA);
        // Create a new transaction
        let inner = self.db.transaction_opt(&wo, &to);
        // The database reference must always outlive
        // the transaction. If it doesn't then this
        // is undefined behaviour. This unsafe block
        // ensures that the transaction reference is
        // static, but will cause a crash if the
        // datastore is dropped prematurely.
        let inner = unsafe {
            std::mem::transmute::<
                rocksdb::Transaction<'_, OptimisticTransactionDB>,
                rocksdb::Transaction<'static, OptimisticTransactionDB>,
            >(inner)
        };
        let mut ro = ReadOptions::default();
        ro.set_snapshot(&inner.snapshot());
        ro.set_async_io(true);
        Ok(Transaction::new(false, write, Some(inner), ro, self.db.clone()))
    }
}


