use std::cmp::Ordering;
use std::ops::Bound;
use serde::{Deserialize, Serialize};
use crate::models::compounds::id::Id;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct IdRange {
    pub beg: Bound<Id>,
    pub end: Bound<Id>,
}

impl PartialOrd for IdRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IdRange {
    fn cmp(&self, other: &Self) -> Ordering {
        match &self.beg {
            Bound::Unbounded => match &other.beg {
                Bound::Unbounded => Ordering::Equal,
                _ => Ordering::Less,
            },
            Bound::Included(v) => match &other.beg {
                Bound::Unbounded => Ordering::Greater,
                Bound::Included(w) => match v.cmp(w) {
                    Ordering::Equal => match &self.end {
                        Bound::Unbounded => match &other.end {
                            Bound::Unbounded => Ordering::Equal,
                            _ => Ordering::Greater,
                        },
                        Bound::Included(v) => match &other.end {
                            Bound::Unbounded => Ordering::Less,
                            Bound::Included(w) => v.cmp(w),
                            _ => Ordering::Greater,
                        },
                        Bound::Excluded(v) => match &other.end {
                            Bound::Excluded(w) => v.cmp(w),
                            _ => Ordering::Less,
                        },
                    },
                    ordering => ordering,
                },
                _ => Ordering::Less,
            },
            Bound::Excluded(v) => match &other.beg {
                Bound::Excluded(w) => match v.cmp(w) {
                    Ordering::Equal => match &self.end {
                        Bound::Unbounded => match &other.end {
                            Bound::Unbounded => Ordering::Equal,
                            _ => Ordering::Greater,
                        },
                        Bound::Included(v) => match &other.end {
                            Bound::Unbounded => Ordering::Less,
                            Bound::Included(w) => v.cmp(w),
                            _ => Ordering::Greater,
                        },
                        Bound::Excluded(v) => match &other.end {
                            Bound::Excluded(w) => v.cmp(w),
                            _ => Ordering::Less,
                        },
                    },
                    ordering => ordering,
                },
                _ => Ordering::Greater,
            },
        }
    }
}