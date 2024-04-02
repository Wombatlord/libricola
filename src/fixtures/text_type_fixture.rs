use std::error::Error;

use sqlx::{query, PgPool};

use crate::domain::text_types::TextType;

#[allow(dead_code)]
pub struct TextTypeFixture;

#[allow(dead_code)]
impl TextTypeFixture {
    pub fn new(text_type: String, text_type_id: i32) -> TextType {
        TextType {
            text_type_id,
            text_type,
        }
    }

    pub async fn create(text_type: &TextType, pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let sql = "INSERT INTO text_types (text_type) VALUES ($1)";

        query(sql).bind(&text_type.text_type).execute(pool).await?;
        Ok(())
    }

    pub async fn populate_text_types(pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let text_types = [
            "Novel",
            "Novella",
            "Short Story",
            "Anthology",
            "Poem",
            "Play",
        ];
        let mut t_vec: Vec<TextType> = vec![];
        for tt_str in text_types {
            t_vec.push(TextTypeFixture::new(tt_str.into(), 0));
        }

        let mut txn = pool.begin().await?;
        let sql = "INSERT INTO text_types (text_type) VALUES ($1)";

        for t in t_vec {
            query(sql).bind(&t.text_type).execute(&mut *txn).await?;
        }

        txn.commit().await?;

        Ok(())
    }
}
