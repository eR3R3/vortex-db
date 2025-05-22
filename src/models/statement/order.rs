use serde::{Deserialize, Serialize};
use crate::models::statement::idiom::Idiom;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub enum Ordering {
    Random,
    Order(OrderList),
}

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct OrderList(pub Vec<Order>);

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct Order {
    /// The value to order by
    pub value: Idiom,
    pub collate: bool,
    pub numeric: bool,
    /// true if the direction is ascending
    pub direction: bool,
}