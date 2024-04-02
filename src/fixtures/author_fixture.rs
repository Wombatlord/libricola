use std::error::Error;

use sqlx::PgPool;

use crate::domain::author::Author;

#[allow(dead_code)]
pub struct AuthorsFixture;

#[allow(dead_code)]
impl AuthorsFixture {
    pub fn new(first_name: String, last_name: String) -> Author {
        Author {
            author_id: None,
            first_name,
            last_name,
        }
    }

    pub async fn populate_authors_table(pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let billy_shakes = AuthorsFixture::new("William".into(), "Shakespeare".into());
        let simpson = AuthorsFixture::new("Homer".into(), "Homer".into());
        let eliot = AuthorsFixture::new("TS".into(), "Eliot".into());
        let pynchon = AuthorsFixture::new("Thomas".into(), "Pynchon".into());
        let banks = AuthorsFixture::new("Iain. M".into(), "Banks".into());

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
