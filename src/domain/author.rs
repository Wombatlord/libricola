use std::error::Error;

use sqlx::{postgres::PgRow, prelude::FromRow, query, PgPool, Row};

#[derive(Debug, FromRow)]
pub struct Author {
    pub author_name: String,
}

impl Author {
    pub fn new(author_name: String) -> Self {
        Self { author_name }
    }

    pub fn row_to_author(row: PgRow) -> Self {
        Self {author_name: row.get("author_name")}        
    }

    pub async fn create(author: &Author, pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let sql = "INSERT INTO authors (author_name) VALUES ($1)";

        query(sql).bind(&author.author_name).execute(pool).await?;
        Ok(())
    }
}
