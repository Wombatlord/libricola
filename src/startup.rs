use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use sqlx::PgPool;

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
    pub async fn build(settings: Settings, pool: PgPool) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port,
        );

        let listener = std::net::TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, pool).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(
    listener: std::net::TcpListener,
    pool: PgPool,
) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(health_check)
            .service(fetch_all_authors)
            .service(fetch_all_text_types)
            .service(fetch_all_text_titles_with_authors)
            .service(fetch_all_text_titles_by_author)
            .service(create_author)
            .service(create_text)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
