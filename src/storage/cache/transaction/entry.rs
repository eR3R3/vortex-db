use std::any::Any;
use std::sync::Arc;
use anyhow::anyhow;
use anyhow::Result;

#[derive(Clone)]
#[non_exhaustive]
pub(crate) enum Entry {
    /// A cached entry of any type
    Any(Arc<dyn Any + Send + Sync>),
    /// A cached record document content
    Val(Arc<Value>),
    /// A slice of DefineDatabaseStatement specified on a namespace.
    Dbs(Arc<[DefineDatabaseStatement]>),
    /// A slice of ApiDefinition specified on a namespace.
    Aps(Arc<[ApiDefinition]>),
    /// A slice of DefineAnalyzerStatement specified on a namespace.
    Azs(Arc<[DefineAnalyzerStatement]>),
    /// A slice of DefineBucketStatement specified on a database.
    Bus(Arc<[BucketDefinition]>),
    /// A slice of DefineFunctionStatement specified on a database.
    Fcs(Arc<[DefineFunctionStatement]>),
    /// A slice of DefineTableStatement specified on a database.
    Tbs(Arc<[DefineTableStatement]>),
    /// A slice of DefineModelStatement specified on a database.
    Mls(Arc<[DefineModelStatement]>),
    /// A slice of DefineConfigStatement specified on a database.
    Cgs(Arc<[DefineConfigStatement]>),
    /// A slice of DefineParamStatement specified on a database.
    Pas(Arc<[DefineParamStatement]>),
    /// A slice of DefineSequenceStatement specified on a namespace.
    Sqs(Arc<[DefineSequenceStatement]>),
    /// A slice of DefineEventStatement specified on a table.
    Evs(Arc<[DefineEventStatement]>),
    /// A slice of DefineFieldStatement specified on a table.
    Fds(Arc<[DefineFieldStatement]>),
    /// A slice of DefineTableStatement specified on a table.
    Fts(Arc<[DefineTableStatement]>),
    /// A slice of DefineIndexStatement specified on a table.
    Ixs(Arc<[DefineIndexStatement]>),
    /// A slice of LiveStatement specified on a table.
    Lvs(Arc<[LiveStatement]>),
}

impl Entry {
    pub(crate) fn try_into_type<T: Send + Sync + 'static>(self: Entry) -> Result<Arc<T>> {
        match self {
            Entry::Any(v) => {
                v.downcast::<T>().map_err(|_| anyhow!("Unable to convert type into Entry::Any"))
            }
            _ => Err(anyhow!("Unable to convert type into Entry::Any")),
        }
    }
}



