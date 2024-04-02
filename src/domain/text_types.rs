use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct TextType {
    pub text_type_id: i32,
    pub text_type: String,
}
