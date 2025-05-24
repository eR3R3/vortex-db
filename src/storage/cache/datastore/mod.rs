mod key;

pub(crate) type Cache = quick_cache::sync::Cache<key::Key, Entry, weight::Weight>;

pub struct DatastoreCache {
    /// Store the cache entries
    cache: Cache,
}

