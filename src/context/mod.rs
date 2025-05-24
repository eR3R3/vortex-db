use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use async_channel::Sender;
use crate::models::compounds::value::Value;
use crate::query::executor::QueryExecutor;
use crate::storage::rocksdb::transaction::Transaction;

pub type Context<'db> = Arc<MutableContext<'db>>;

#[non_exhaustive]
pub struct MutableContext<'db> {
    // An optional parent context.
    parent: Option<Context<'db>>,
    // An optional deadline.
    deadline: Option<Instant>,
    // Whether this context is cancelled.
    cancelled: Arc<AtomicBool>,
    // A collection of read only values stored in this context.
    values: HashMap<Cow<'static, str>, Arc<Value>>,
    // Stores the notification channel if available
    notifications: Option<Sender<Notification>>,
    // An optional query planner
    query_planner: Option<Arc<QueryPlanner>>,
    // An optional query executor
    query_executor: Option<QueryExecutor>,
    // An optional iteration stage
    iteration_stage: Option<IterationStage>,
    // An optional datastore cache
    cache: Option<Arc<DatastoreCache>>,
    // The index store
    index_stores: IndexStores,
    // The index concurrent builders
    index_builder: Option<IndexBuilder>,
    // The sequences
    sequences: Option<Sequences>,
    // Capabilities
    capabilities: Arc<Capabilities>,
    temporary_directory: Option<Arc<PathBuf>>,
    // An optional transaction
    transaction: Option<Arc<Transaction<'db>>>,
    // Does not read from parent `values`.[
    isolated: bool,
    // A map of bucket connections
    buckets: Option<Arc<BucketConnections>>,
}

impl MutableContext<'_> {
    pub(crate) fn tx(&self) -> Arc<Transaction> {
        self.transaction
            .clone()
            .unwrap_or_else(|| unreachable!("The context was not associated with a transaction"))
    }
}