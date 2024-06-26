mod domain;
mod fixtures;
mod get;
mod post;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use fixtures::{
    author_fixture::AuthorsFixture, text_fixture::TextFixtures, text_type_fixture::TextTypeFixture,
};
use get::get_endpoints::{
    fetch_all_authors, fetch_all_text_titles_by_author, fetch_all_text_titles_with_authors,
    fetch_all_text_types,
};
use post::post_endpoints::{create_author, create_text};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::error::Error;

// $ docker run -e POSTGRES_PASSWORD=123456 -e POSTGRES_USER=user -e POSTGRES_DB=libricola -p 5432:5432 postgres

struct AppState {
    db: Pool<Postgres>,
}

#[allow(dead_code)]
async fn bootstrap_some_data(pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    TextTypeFixture::populate_text_types(&pool).await?;
    AuthorsFixture::populate_authors_table(&pool).await?;
    TextFixtures::populate_shakespeare(&pool).await?;
    TextFixtures::populate_homer(&pool).await?;
    TextFixtures::populate_eliot(&pool).await?;
    TextFixtures::populate_pynchon(&pool).await?;
    TextFixtures::populate_banks(&pool).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error building connection pool");

    sqlx::migrate!("./migrations").run(&pool).await?;
    // bootstrap_some_data(&pool).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(fetch_all_authors)
            .service(fetch_all_text_types)
            .service(fetch_all_text_titles_with_authors)
            .service(fetch_all_text_titles_by_author)
            .service(create_author)
            .service(create_text)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
