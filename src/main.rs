// mod domain;

use dotenv::dotenv;

use libricola::{
    fixtures::{
        author_fixture::AuthorsFixture, text_fixture::TextFixtures,
        text_type_fixture::TextTypeFixture,
    },
    settings, startup, telemetry,
};
use sqlx::{Pool, Postgres};
use std::error::Error;

// $ docker run -e POSTGRES_PASSWORD=123456 -e POSTGRES_USER=user -e POSTGRES_DB=libricola -p 5432:5432 postgres

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

    let settings = settings::get_settings().expect("Failed to read settings.");
    let subscriber = telemetry::get_subscriber(settings.clone().debug);
    telemetry::init_subscriber(subscriber);

    let application = startup::Application::build(settings, None).await?;
    
    tracing::event!(target: "libricola", tracing::Level::INFO, "Listening on http://127.0.0.1:{}/", application.port());

    application.run_until_stopped().await?;

    Ok(())
}
