use std::error::Error;

use sqlx::PgPool;

use crate::domain::author::Author;

pub struct AuthorsFixture;

impl AuthorsFixture {
    pub async fn populate_authors_table(pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let billy_shakes = Author::new("Shakespeare, William".into());
        let simpson = Author::new("Homer".into());
        let eliot = Author::new("Eliot, T.S".into());
        let pynchon = Author::new("Pynchon, Thomas".into());
        let banks = Author::new("Banks, Iain. M".into());

        let authors = [billy_shakes, simpson, eliot, pynchon, banks];
    
        let mut txn = pool.begin().await?;
        let sql = "INSERT INTO authors (author_name) VALUES ($1)";
    
        for author in authors {
            sqlx::query(sql).bind(&author.author_name).execute(&mut *txn).await?;
        }
    
        txn.commit().await?;
        Ok(())
    }
}