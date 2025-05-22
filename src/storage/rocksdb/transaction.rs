use std::sync::Arc;
use rocksdb::{OptimisticTransactionDB, ReadOptions};
use anyhow::{anyhow, bail, Result};

pub struct Transaction {
    /// Is the transaction complete?
    done: bool,
    /// Is the transaction writeable?
    write: bool,
    /// The underlying datastore transaction
    inner: Option<rocksdb::Transaction<'static, OptimisticTransactionDB>>,
    /// The read options containing the Snapshot
    ro: ReadOptions,
    // The above, supposedly 'static transaction
    // actually points here, so we need to ensure
    // the memory is kept alive. This pointer must
    // be declared last, so that it is dropped last.
    _db: Arc<OptimisticTransactionDB>,
}


impl Transaction {
    pub fn new(done: bool, write: bool,
               inner: Option<rocksdb::Transaction<'static, OptimisticTransactionDB>>,
               ro: ReadOptions, db: Arc<OptimisticTransactionDB>) -> Self {
        Transaction {
            done,
            write,
            inner,
            ro,
            _db: db
        }
    }
    
    pub fn as_ref(&mut self) -> Result<&rocksdb::Transaction<OptimisticTransactionDB>> {
        let db_tx = self
            .inner
            .as_ref()
            .ok_or_else(|| {
                self.done = true;
                anyhow!("Transaction already committed")
            })?;
        Ok(db_tx)
    }

    pub fn as_mut_ref(&mut self) -> Result<&rocksdb::Transaction<OptimisticTransactionDB>> {
        let db_tx = self
            .inner
            .as_mut()
            .ok_or_else(|| {
                self.done = true;
                anyhow!("Transaction already committed")
            })?;
        Ok(db_tx)
    }

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let db_tx = self
            .inner
            .as_ref()
            .ok_or_else(|| anyhow!("Transaction already committed"))?;

        db_tx.get(key).map_err(|e| anyhow!("{}", e))
    }
}