use serde::Serialize;
use sqlx::{Decode, FromRow, Type};

#[derive(Debug, FromRow, Serialize, Type, Decode)]
pub struct ExpenseRecord {
    pub amount: f64,
    pub category: String,
    pub datetime: i64,
    pub tags: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct ExpenseTotal {
    pub total: f64,
}

// impl Decode for ExpenseRecord {
//     fn decode(value: <DB as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {

//     }
// }
