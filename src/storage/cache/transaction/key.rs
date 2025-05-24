use uuid::Uuid;
use crate::models::compounds::id::Id;

#[derive(Clone, Hash, Eq, PartialEq)]
pub(crate) enum Key {
    /// A cache key for databases
    Dbs(String),
    /// A cache key for database users
    Dus(String, String),
    /// A cache key for database accesses
    Das(String, String),
    /// A cache key for apis (on a database)
    Aps(String, String),
    /// A cache key for analyzers (on a database)
    Azs(String, String),
    /// A cache key for buckets (on a database)
    Bus(String, String),
    /// A cache key for functions (on a database)
    Fcs(String, String),
    /// A cache key for models (on a database)
    Mls(String, String),
    /// A cache key for configs (on a database)
    Cgs(String, String),
    /// A cache key for parameters (on a database)
    Pas(String, String),
    /// A cache key for tables
    Tbs(String, String),
    /// A cache key for sequences (on a database)
    Seq(String, String),
    /// A cache key for events (on a table)
    Evs(String, String, String),
    /// A cache key for fieds (on a table)
    Fds(String, String, String),
    /// A cache key for views (on a table)
    Fts(String, String, String),
    /// A cache key for indexes (on a table)
    Ixs(String, String, String),
    /// A cache key for live queries (on a table)
    Lvs(String, String, String),
    /// A cache key for a node
    Nd(Uuid),
    /// A cache key for a database
    Db(String, String),
    /// A cache key for an api (on a database)
    Ap(String, String, String),
    /// A cache key for an analyzer (on a database)
    Az(String, String, String),
    /// A cache key for a bucket (on a database)
    Bu(String, String, String),
    /// A cache key for a function (on a database)
    Fc(String, String, String),
    /// A cache key for a model (on a database)
    Ml(String, String, String, String),
    /// A cache key for a config (on a database)
    Cg(String, String, String),
    /// A cache key for a parameter (on a database)
    Pa(String, String, String),
    /// A cache key for a sequence (on a database)
    Sq(String, String, String),
    /// A cache key for a table
    Tb(String, String, String),
    /// A cache key for an event (on a table)
    Ev(String, String, String, String),
    /// A cache key for a field (on a table)
    Fd(String, String, String, String),
    /// A cache key for an index (on a table)
    Ix(String, String, String, String),
    /// A cache key for a record
    Record(String, String, String, Id),
}