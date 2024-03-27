use std::error::Error;

use sqlx::PgPool;

use crate::domain::author::Author;

pub struct AuthorsFixture;

impl AuthorsFixture {
    pub async fn populate_authors_table(pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let billy_shakes = Author::new("William".into(), "Shakespeare".into());
        let simpson = Author::new("Homer".into(), "Homer".into());
        let eliot = Author::new("TS".into(), "Eliot".into());
        let pynchon = Author::new("Thomas".into(), "Pynchon".into());
        let banks = Author::new("Iain. M".into(), "Banks".into());

        let authors = [billy_shakes, simpson, eliot, pynchon, banks];

        let mut txn = pool.begin().await?;
        let sql = "INSERT INTO authors (first_name, last_name) VALUES ($1, $2)";

        for author in authors {
            sqlx::query(sql)
                .bind(&author.first_name)
                .bind(&author.last_name)
                .execute(&mut *txn)
                .await?;
        }

        txn.commit().await?;
        Ok(())
    }
}
