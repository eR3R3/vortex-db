use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use async_channel::{Receiver, Sender};
use uuid::Uuid;


pub(super) enum DatastoreFlavor {
    #[cfg(feature = "kv-mem")]
    Mem(super::mem::Datastore),
    #[cfg(feature = "kv-rocksdb")]
    RocksDB(super::rocksdb::datastore::Datastore),
}

#[derive(Clone)]
pub(super) struct TransactionFactory {
    // Clock for tracking time. It is read only and accessible to all transactions. It is behind a mutex as tests may write to it.
    clock: Arc<SizedClock>,
    // The inner datastore type
    flavor: Arc<DatastoreFlavor>,
}
pub struct Datastore {
    transaction_factory: TransactionFactory,
    /// The unique id of this datastore, used in notifications.
    id: Uuid,
    /// Whether authentication is enabled on this datastore.
    auth_enabled: bool,
    /// The maximum duration timeout for running multiple statements in a query.
    query_timeout: Option<Duration>,
    /// The maximum duration timeout for running multiple statements in a transaction.
    transaction_timeout: Option<Duration>,
    /// The security and feature capabilities for this datastore.
    capabilities: Arc<Capabilities>,
    // Whether this datastore enables live query notifications to subscribers.
    notification_channel: Option<(Sender<Notification>, Receiver<Notification>)>,
    // The index store cache
    index_stores: IndexStores,
    // The cross transaction cache
    cache: Arc<DatastoreCache>,
    // The index asynchronous builder
    index_builder: IndexBuilder,
    // The JWKS object cache
    jwks_cache: Arc<RwLock<JwksCache>>,
    // The temporary directory
    temporary_directory: Option<Arc<PathBuf>>,
    // The sequences
    sequences: Sequences,
}