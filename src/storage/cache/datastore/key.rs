use uuid::Uuid;

#[derive(Clone, Hash, Eq, PartialEq)]
pub(crate) enum Key {
    /// A cache key for a database
    Db(String, String),
    /// A cache key for a table
    Tb(String, String, String),
    /// A cache key for events (on a table)
    Evs(String, String, String, Uuid),
    /// A cache key for fieds (on a table)
    Fds(String, String, String, Uuid),
    /// A cache key for views (on a table)
    Fts(String, String, String, Uuid),
    /// A cache key for indexes (on a table)
    Ixs(String, String, String, Uuid),
    /// A cache key for live queries (on a table)
    Lvs(String, String, String, Uuid),
    /// A cache key for live queries version (on a table)
    Lvv(String, String, String),
}

