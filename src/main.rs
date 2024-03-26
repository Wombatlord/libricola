mod domain;
mod fixtures;

use domain::{
    author::Author,
    text::{Metadata, Text},
    text_types::TextType,
};
use fixtures::{authors::AuthorsFixture, texts::TextFixtures};
use sqlx::FromRow;
use std::error::Error;

use crate::domain::db::Db;

// $ docker run -e POSTGRES_PASSWORD=123456 -e POSTGRES_USER=user -e POSTGRES_DB=libricola -p 5432:5432 postgres

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://libricola:123456@localhost:5432/libricola";
    let db = Db::with_connection(url).await?;
    sqlx::migrate!("./migrations").run(&db.connection).await?;

    // TextType::populate_text_types(&pool).await?;
    // AuthorsFixture::populate_authors_table(&pool).await?;
    // TextFixtures::populate_shakespeare(&pool).await?;
    // TextFixtures::populate_homer(&pool).await?;
    // TextFixtures::populate_eliot(&pool).await?;
    // TextFixtures::populate_pynchon(&pool).await?;
    // TextFixtures::populate_banks(&pool).await?;

    let q = "SELECT authors.author_name FROM authors WHERE authors.author_name = 'Pynchon, Thomas'";
    let pynch: Author = sqlx::query_as(q).fetch_one(&db.connection).await?;

    println!("{:?}", pynch);

    let q = "SELECT * FROM texts JOIN text_types ON texts.text_type_id = text_types.text_type_id JOIN authors ON texts.author_id = authors.author_id WHERE authors.author_name = 'Pynchon, Thomas'";
    let book: Text = sqlx::query_as(q).fetch_one(&db.connection).await?;

    println!("{:?}", book);
    Ok(())
}
