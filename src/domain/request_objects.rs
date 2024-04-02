use super::text::Metadata;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorRequestBody {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TextRequestBody {
    pub text_type: String,
    pub title: String,
    pub published: i32,
    #[sqlx(json)]
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTextRequest {
    pub text: TextRequestBody,
    pub author: AuthorRequestBody,
}

#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct TitleWithAuthor {
    pub title: String,
    pub first_name: String,
    pub last_name: String,
    pub published: i32,
}
