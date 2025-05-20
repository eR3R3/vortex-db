use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash, Ord)]
pub enum Number {
    #[default]
    Int(i64),
    Float(f64),
    Decimal(Decimal),
    // Add new variants here
}