use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::context::Context;
use crate::models::compounds::document::CursorDoc;
use crate::models::compounds::value::Value;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct DefineDatabaseStatement {
    pub id: Option<u32>,
    pub name: String,
    pub comment: Option<String>,
    pub overwrite: bool,
}

impl DefineDatabaseStatement {
    pub(crate) async fn compute(
        &self,
        ctx: &Context,
        opt: &Options,
        _doc: Option<&CursorDoc>,
    ) -> Result<Value> {
        // Fetch the transaction
        let transaction = ctx.tx();
        if transaction.get_db(&self.name).await.is_ok() {
            if self.if_not_exists {
                return Ok(Value::None);
            } else if !self.overwrite {
                return Err(Error::DbAlreadyExists {
                    name: self.name.to_string(),
                });
            }
        }
    }
}
