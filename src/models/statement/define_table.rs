use uuid::Uuid;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]

#[non_exhaustive]
pub struct DefineTableStatement {
    pub id: Option<u32>,
    pub name: String,
    pub drop: bool,
    pub full: bool,
    pub view: Option<View>,
    pub permissions: Permissions,
    pub changefeed: Option<ChangeFeed>,
    pub comment: Option<String>,
    pub if_not_exists: bool,
    pub kind: TableType,
    /// Should we overwrite the field definition if it already exists
    pub overwrite: bool,
    /// The last time that a DEFINE FIELD was added to this table
    pub cache_fields_ts: Uuid,
    /// The last time that a DEFINE EVENT was added to this table
    pub cache_events_ts: Uuid,
    /// The last time that a DEFINE TABLE was added to this table
    pub cache_tables_ts: Uuid,
    /// The last time that a DEFINE INDEX was added to this table
    pub cache_indexes_ts: Uuid,
    /// The last time that a LIVE query was added to this table
    pub cache_lives_ts: Uuid,
}