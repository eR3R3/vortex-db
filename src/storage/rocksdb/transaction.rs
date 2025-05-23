use std::sync::Arc;
use rocksdb::{OptimisticTransactionDB, ReadOptions};
use anyhow::{anyhow, Result};
use crate::models::compounds::tuple::Tuple;
use crate::models::compounds::value::Value;
use crate::models::statement::define_database::DefineDatabaseStatement;
use crate::storage::api::key::KeyEncode;
use crate::storage::cache;
use crate::storage::cache::transaction::TransactionCache;

pub struct Transaction<'db> {
    // cache for the transaction
    cache: TransactionCache,
    inner: TransactionRaw<'db>,
}

impl<'db> Transaction<'db> {
    pub fn get_db(&self, db: &str) -> Result<Arc<DefineDatabaseStatement>> {
        let qey = cache::transaction::lookup::Lookup::Db(db);
        match self.cache.get(&qey) {
            Some(val) => val.try_into_type(),
            None => {
                // the encoded key
                let key = crate::key::db::new(db).encode()?;
                let val = self.get(key)?.ok_or_else(anyhow!("db not found"))?;
                let val: DefineDatabaseStatement = revision::from_slice(&val)?;
                let val = Arc::new(val);
                let entry = cache::transaction::Entry::Any(val.clone());
                self.cache.insert(qey, entry);
                Ok(val)
            }
        }
    }
    
    pub fn get<T: KeyEncode>(&self, key: T) -> Result<Option<Vec<u8>>> {
        self.inner.get(key)
    }
}

pub struct TransactionRaw<'db> {
    /// Is the transaction writeable?
    write: bool,
    /// The underlying datastore transaction
    inner: Option<rocksdb::Transaction<'db, OptimisticTransactionDB>>,
    /// The read options containing the Snapshot
    ro: ReadOptions,
}


impl<'db> TransactionRaw<'db> {
    pub fn new(write: bool,
               inner: Option<rocksdb::Transaction<'db, OptimisticTransactionDB>>,
               ro: ReadOptions) -> Self {
        TransactionRaw {
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
    fn get<T: KeyEncode>(&self, raw_key: T) -> Result<Option<Vec<u8>>> {
        let key = raw_key.encode()?;
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
        let db_tx = self.inner_ref().unwrap();

    }
}

pub(crate) struct RocksDbIterator<'db, 'a>
where
    'db: 'a,
{
    inner: rocksdb::DBIteratorWithThreadMode<'a, rocksdb::Transaction<'db, OptimisticTransactionDB>>,
    upper_bound: Vec<u8>,
}

impl<'db, 'a> Iterator for RocksDbIterator<'db, 'a> {
    type Item = Result<Tuple>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(result) = self.inner.next() {
            match result {
                Ok((k, v)) => {
                    if k.as_ref() >= self.upper_bound.as_slice() {
                        return None;
                    }
                    return Some(Ok(decode_tuplem_kv(&k, &v, None)));
                }
                Err(e) => return Some(Err(anyhow!("Iterator error: {}", e))),
            }
        }
        None
    }
}