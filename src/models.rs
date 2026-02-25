use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{Utc, DateTime};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ExpenseRecord {
    pub amount: f64,
    pub category: String,
    pub tags: Vec<String>,
    pub datetime: DateTime<Utc>,
    pub description: Option<String>,
}