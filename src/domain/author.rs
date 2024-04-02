use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Author {
    pub author_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
}
