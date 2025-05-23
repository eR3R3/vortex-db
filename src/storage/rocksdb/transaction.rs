use std::sync::Arc;
use rocksdb::{OptimisticTransactionDB, ReadOptions};
use anyhow::{anyhow, Result};

pub struct Transaction<'db> {
    /// Is the transaction writeable?
    write: bool,
    /// The underlying datastore transaction
    inner: Option<rocksdb::Transaction<'db, OptimisticTransactionDB>>,
    /// The read options containing the Snapshot
    ro: ReadOptions,
}


impl<'db> Transaction<'db> {
    pub fn new(write: bool,
               inner: Option<rocksdb::Transaction<'static, OptimisticTransactionDB>>,
               ro: ReadOptions) -> Self {
        Transaction {
            write,
            inner,
            ro,
        }
    }
    
    pub fn inner_ref(&self) -> Result<&rocksdb::Transaction<OptimisticTransactionDB>> {
        let db_tx = self
            .inner
            .as_ref()
            .ok_or_else(|| {
                anyhow!("Transaction already committed")
            })?;
        Ok(db_tx)
    }

    pub fn inner_mut_ref<'a>(&'a mut self) -> Result<&'a mut rocksdb::Transaction<'db, OptimisticTransactionDB>> {
        let db_tx = self
            .inner
            .as_mut()
            .ok_or_else(|| {
                anyhow!("Transaction already committed")
            })?;
        Ok(db_tx)
    }

    // get a key from a transaction
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let db_tx = self.inner_ref()?;
        db_tx.get(key).map_err(|e| anyhow!("{}", e))
    }

    fn put(&mut self, key: &[u8], val: &[u8]) -> Result<()> {
        let db_tx = self.inner_mut_ref()?;
        db_tx.put(key, val)?;
        Ok(())
    }
    
    fn delete(&mut self, key: &[u8]) -> Result<()> {
        let db_tx = self.inner_mut_ref()?;
        db_tx.delete(key)?;
        Ok(())
    }
    
    #[inline]
    fn par_put(&mut self, key: &[u8], val: &[u8]) -> Result<()> {
        let db_tx = self.inner_ref()?;
        db_tx.put(key, val)?;
        Ok(())
    }
    
    #[inline]
    fn par_delete(&mut self, key: &[u8]) -> Result<()> {
        let db_tx = self.inner_ref()?;
        db_tx.delete(key)?;
        Ok(())
    }

    fn del_range_from_persisted(&mut self, lower: &[u8], upper: &[u8]) -> Result<()>  {
        let db_tx = self.inner_mut_ref()?;
        let iter = db_tx
            .iterator(rocksdb::IteratorMode::From(lower, rocksdb::Direction::Forward));
        for item in iter {
            let (k, _) = item?;
            if k >= upper.into() {
                break;
            }
            db_tx.delete(&k)?;
        }
        Ok(())
    }

    #[inline]
    fn exists(&self, key: &[u8]) -> Result<bool> {
        let db_tx = self.inner_ref()?;
        Ok(db_tx.get(key)?.is_some())
    }

    fn commit(&mut self) -> Result<()> {
        let db_tx = self.inner.take().expect("Transaction already committed");
        Ok(db_tx.commit()?)
    }

    fn range_scan_tuple<'a>(
        &'a self,
        lower: &[u8],
        upper: &[u8],
    ) -> Box<dyn Iterator<Item = Result<Tuple>> + 'a>
    {
        match &self.db_tx {
            Some(db_tx) => Box::new(NewRocksDbIterator {
                inner: db_tx.iterator(rocksdb::IteratorMode::From(
                    lower,
                    rocksdb::Direction::Forward,
                )),
                upper_bound: upper.to_vec(),
            }),
            None => Box::new(std::iter::once(Err(miette!(
                "Transaction already committed"
            )))),
        }
    }
}