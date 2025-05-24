use std::borrow::Borrow;
pub(crate) use crate::storage::cache::transaction::entry::Entry;
use crate::storage::cache::transaction::lookup::Lookup;

pub mod key;
pub mod entry;
pub mod lookup;

pub(crate) type Cache = quick_cache::sync::Cache<key::Key, Entry, weight::Weight>;

pub struct TransactionCache {
    /// Store the cache entries
    cache: Cache,
}

impl TransactionCache {
    pub(crate) fn get(&self, lookup: &Lookup) -> Option<Entry> {
        self.cache.get(lookup)
    }
}
