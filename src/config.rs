use once_cell::sync::Lazy;

pub type Key = Vec<u8>;
pub type Val = Vec<u8>;
pub type Version = u64;


/// The number of threads to start for flushing and compaction (default: number of CPUs)
pub(crate) static ROCKSDB_THREAD_COUNT: Lazy<i32> = Lazy::new(|| num_cpus::get() as i32);

/// The maximum number of threads to use for flushing and compaction (default: number of CPUs * 2)
pub(crate) static ROCKSDB_JOBS_COUNT: Lazy<i32> = Lazy::new(|| num_cpus::get() as i32 * 2);

/// The maximum number of open files which can be opened by RocksDB (default: 1024)
pub(crate) static ROCKSDB_MAX_OPEN_FILES: i32 = 1024;

pub(super) static ROCKSDB_KEEP_LOG_FILE_NUM: usize = 20;

pub(super) static ROCKSDB_MAX_WRITE_BUFFER_NUMBER: i32 = 32;

// Each Memtable Size
pub(super) static ROCKSDB_WRITE_BUFFER_SIZE: usize = 256 * 1024 * 1024;

pub(super) static ROCKSDB_TARGET_FILE_SIZE_BASE: u64 = 128 * 1024 * 1024;

pub(super) static ROCKSDB_TARGET_FILE_SIZE_MULTIPLIER: i32 = 10;

pub(super) static ROCKSDB_MIN_WRITE_BUFFER_NUMBER_TO_MERGE: i32 = 6;

pub(super) static ROCKSDB_FILE_COMPACTION_TRIGGER: i32 = 16;

pub(super) static ROCKSDB_COMPACTION_READAHEAD_SIZE: usize = 16 * 1024 * 1024;

pub(super) static ROCKSDB_MAX_CONCURRENT_SUBCOMPACTIONS: u32 = 4;

pub(super) static ROCKSDB_MIN_BLOB_SIZE: u64 =  4 * 1024;

pub(super) static ROCKSDB_WAL_SIZE_LIMIT: u64 = 1024;

pub(crate) static ROCKSDB_BLOCK_CACHE_SIZE: usize =  512 * 1024 * 1024;

pub(crate) static ROCKSDB_BLOCK_SIZE: usize = 64 * 1024;

pub(crate) static ROCKSDB_ENABLE_MEMORY_MAPPED_READS: bool = false;

pub(super) static ROCKSDB_BACKGROUND_FLUSH: bool = false;

pub(crate) static ROCKSDB_BACKGROUND_FLUSH_INTERVAL: u64 = 200;

// determine whether the data flush to disk directly(slow but safe) or keep that in memory for now
// and let OS determine when to flush to disk(quick but unsafe)
pub(crate) static SYNC_DATA: bool = true;

pub(crate) static NORMAL_FETCH_SIZE: u32 = 500;