use std::error::Error;

use sqlx::PgPool;

pub struct Db {
    pub connection: PgPool,
}

impl Db {
    pub async fn with_connection(url: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            connection: Db::connect(url).await?,
        })
    }

    pub async fn connect(url: &str) -> Result<PgPool, Box<dyn Error>> {
        Ok(sqlx::postgres::PgPool::connect(url).await?) 
    }
}
