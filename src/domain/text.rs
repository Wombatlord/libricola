use std::error::Error;

use serde::{Deserialize, Serialize};
use sqlx::{query, types::Json, FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Metadata {
    pub genre_tags: Vec<String>,
}

#[derive(Debug, FromRow)]
pub struct Text {
    pub text_type_id: i32,
    pub author_id: i32,
    pub title: String,
    pub published: i32,
    #[sqlx(json)]
    pub metadata: Metadata,
}

impl Text {
    pub fn new(
        text_type_id: i32,
        author_id: i32,
        title: String,
        published: i32,
        metadata: Metadata,
    ) -> Self {
        Self {
            text_type_id,
            author_id,
            title,
            published,
            metadata,
        }
    }

    pub async fn create(text: &Text, pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let sql =
            "INSERT INTO texts (text_type_id, author_id, title, published, metadata) VALUES ($1, $2, $3, $4, $5)";
       
        query(sql)
            .bind(&text.text_type_id)
            .bind(&text.author_id)
            .bind(&text.title)
            .bind(&text.published)
            .bind(Json(&text.metadata))
            .execute(pool)
            .await?;
        Ok(())
    }
}
