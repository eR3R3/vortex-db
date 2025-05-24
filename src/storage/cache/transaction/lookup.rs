use quick_cache::Equivalent;
use uuid::Uuid;
use crate::models::compounds::id::Id;
use crate::storage::cache::transaction::key::Key;

#[derive(Hash, Eq, PartialEq)]
pub(crate) enum Lookup<'a> {
    /// A cache key for databases
    Dbs(&'a str),
    /// A cache key for apis (on a database)
    Aps(&'a str, &'a str),
    /// A cache key for analyzers (on a database)
    Azs(&'a str, &'a str),
    /// A cache key for buckets (on a database)
    Bus(&'a str, &'a str),
    /// A cache key for functions (on a database)
    Fcs(&'a str, &'a str),
    /// A cache key for models (on a database)
    Mls(&'a str, &'a str),
    /// A cache key for configs (on a database)
    Cgs(&'a str, &'a str),
    /// A cache key for parameters (on a database)
    Pas(&'a str, &'a str),
    /// A cache key for sequences (on a database)
    Sqs(&'a str, &'a str),
    /// A cache key for tables
    Tbs(&'a str, &'a str),
    /// A cache key for events (on a table)
    Evs(&'a str, &'a str, &'a str),
    /// A cache key for fields (on a table)
    Fds(&'a str, &'a str, &'a str),
    /// A cache key for views (on a table)
    Fts(&'a str, &'a str, &'a str),
    /// A cache key for indexes (on a table)
    Ixs(&'a str, &'a str, &'a str),
    /// A cache key for live queries (on a table)
    Lvs(&'a str, &'a str, &'a str),
    /// A cache key for a database
    Db(&'a str),
    /// A cache key for an api (on a database)
    Ap(&'a str, &'a str, &'a str),
    /// A cache key for an analyzer (on a database)
    Az(&'a str, &'a str, &'a str),
    /// A cache key for a bucket (on a database)
    Bu(&'a str, &'a str, &'a str),
    /// A cache key for a function (on a database)
    Fc(&'a str, &'a str, &'a str),
    /// A cache key for a model (on a database)
    Ml(&'a str, &'a str, &'a str, &'a str),
    /// A cache key for a config (on a database)
    Cg(&'a str, &'a str, &'a str),
    /// A cache key for a parameter (on a database)
    Pa(&'a str, &'a str, &'a str),
    /// A cache key for a sequence (on a database)
    Sq(&'a str, &'a str, &'a str),
    /// A cache key for a table
    Tb(&'a str, &'a str, &'a str),
    /// A cache key for an event (on a table)
    Ev(&'a str, &'a str, &'a str, &'a str),
    /// A cache key for a field (on a table)
    Fd(&'a str, &'a str, &'a str, &'a str),
    /// A cache key for an index (on a table)
    Ix(&'a str, &'a str, &'a str, &'a str),
    /// A cache key for a record
    Record(&'a str, &'a str, &'a str, &'a Id),
}



impl Equivalent<Key> for Lookup<'_> {
    #[rustfmt::skip]
	fn equivalent(&self, key: &Key) -> bool {
		match (self, key) {
			(Self::Dbs(la), Key::Dbs(ka)) => la == ka,
			(Self::Aps(la, lb), Key::Aps(ka, kb)) => la == ka && lb == kb,
			(Self::Azs(la, lb), Key::Azs(ka, kb)) => la == ka && lb == kb,
			(Self::Bus(la, lb), Key::Bus(ka, kb)) => la == ka && lb == kb,
			(Self::Fcs(la, lb), Key::Fcs(ka, kb)) => la == ka && lb == kb,
			(Self::Mls(la, lb), Key::Mls(ka, kb)) => la == ka && lb == kb,
			(Self::Cgs(la, lb), Key::Cgs(ka, kb)) => la == ka && lb == kb,
			(Self::Pas(la, lb), Key::Pas(ka, kb)) => la == ka && lb == kb,
			(Self::Tbs(la, lb), Key::Tbs(ka, kb)) => la == ka && lb == kb,
			(Self::Evs(la, lb, lc), Key::Evs(ka, kb, kc)) => la == ka && lb == kb && lc == kc,
			(Self::Fds(la, lb, lc), Key::Fds(ka, kb, kc)) => la == ka && lb == kb && lc == kc,
			(Self::Fts(la, lb, lc), Key::Fts(ka, kb, kc)) => la == ka && lb == kb && lc == kc,
			(Self::Ixs(la, lb, lc), Key::Ixs(ka, kb, kc)) => la == ka && lb == kb && lc == kc,
			(Self::Lvs(la, lb, lc), Key::Lvs(ka, kb, kc)) => la == ka && lb == kb && lc == kc,
			//
			(Self::Db( lb), Key::Db(ka, kb)) =>  lb == kb,
			(Self::Ap(la, lb, lc), Key::Ap(ka, kb, kc)) => la == ka && lb == kb && lc == kc,
			(Self::Az(la, lb, lc), Key::Az(ka, kb, kc)) => la == ka && lb == kb && lc == kc,
			(Self::Bu(la, lb, lc), Key::Bu(ka, kb, kc)) => la == ka && lb == kb && lc == kc,
			(Self::Fc(la, lb, lc), Key::Fc(ka, kb, kc)) => la == ka && lb == kb && lc == kc,
			(Self::Ml(la, lb, lc, ld), Key::Ml(ka, kb, kc, kd)) => la == ka && lb == kb && lc == kc && ld == kd,
			(Self::Cg(la, lb, lc), Key::Cg(ka, kb, kc)) => la == ka && lb == kb && lc == kc,
			(Self::Pa(la, lb, lc), Key::Pa(ka, kb, kc)) => la == ka && lb == kb && lc == kc,
			(Self::Tb(la, lb, lc), Key::Tb(ka, kb, kc)) => la == ka && lb == kb && lc == kc,
			(Self::Ev(la, lb, lc, ld), Key::Ev(ka, kb, kc, kd)) => la == ka && lb == kb && lc == kc && ld == kd,
			(Self::Fd(la, lb, lc, ld), Key::Fd(ka, kb, kc, kd)) => la == ka && lb == kb && lc == kc && ld == kd,
			(Self::Ix(la, lb, lc, ld), Key::Ix(ka, kb, kc, kd)) => la == ka && lb == kb && lc == kc && ld == kd,
			(Self::Record(la, lb, lc, ld), Key::Record(ka, kb, kc, kd)) => la == ka && lb == kb && lc == kc && *ld == kd,
			//
			_ => false,
		}
	}
}
