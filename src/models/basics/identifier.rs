use std::io::{Cursor, Write};
use internment::Intern;
use anyhow::Result;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Ord, PartialOrd)]
pub struct Identifier(pub(crate) Intern<String>);

impl Identifier {
    pub unsafe fn new_unchecked(raw: String) -> Identifier {
        Identifier(Intern::new(raw))
    }
}