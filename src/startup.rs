use crate::{
    app_state::AppState,
    routes::{
        create_author, create_text, fetch_all_authors, fetch_all_text_titles_by_author,
        fetch_all_text_titles_with_authors, fetch_all_text_types, health_check,
    },
    settings::Settings,
};

pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build(
        settings: Settings,
        test_pool: Option<sqlx::postgres::PgPool>,
    ) -> Result<Self, std::io::Error> {
        let connection_pool = if let Some(pool) = test_pool {
            pool
        } else {
            get_connection_pool(&settings.database).await
        };

        sqlx::migrate!()
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database.");
        // bootstrap_some_data(&pool).await?;

        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port,
        );

        let listener = std::net::TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn get_connection_pool(
    settings: &crate::settings::DatabaseSettings,
) -> sqlx::Pool<sqlx::Postgres> {
    // let db_url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL.");
    match sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with(settings.connect_to_db())
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Couldn't establish DB connection!: {:#?}", e);
            panic!("Couldn't establish DB connection!")
        }
    }
}

async fn run(
    listener: std::net::TcpListener,
    db_pool: sqlx::postgres::PgPool,
) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .service(health_check)
            .service(fetch_all_authors)
            .service(fetch_all_text_types)
            .service(fetch_all_text_titles_with_authors)
            .service(fetch_all_text_titles_by_author)
            .service(create_author)
            .service(create_text)
            .app_data(actix_web::web::Data::new(AppState {
                db: db_pool.clone(),
            }))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
