use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};

use crate::{
    domain::{
        author::Author,
        text::TitleWithAuthor,
        text_types::TextType,
    },
    AppState,
};

#[get("/authors")]
pub async fn fetch_all_authors(state: Data<AppState>) -> impl Responder {
    // "GET /authors".to_string()
    let sql = "SELECT first_name, last_name FROM authors";
    match sqlx::query_as::<_, Author>(sql).fetch_all(&state.db).await {
        Ok(authors) => HttpResponse::Ok().json(authors),
        Err(_) => HttpResponse::NotFound().json("No authors found."),
    }
}

#[get("/text_types")]
pub async fn fetch_all_text_types(state: Data<AppState>) -> impl Responder {
    let sql = "SELECT text_type FROM text_types";
    match sqlx::query_as::<_, TextType>(sql)
        .fetch_all(&state.db)
        .await
    {
        Ok(tt) => HttpResponse::Ok().json(tt),
        Err(_) => HttpResponse::NotFound().json("No text types found."),
    }
}

#[get("/texts")]
pub async fn fetch_all_text_titles_with_authors(state: Data<AppState>) -> impl Responder {
    let sql = "SELECT texts.title, authors.first_name, authors.last_name FROM texts JOIN authors ON texts.author_id = authors.author_id";
    match sqlx::query_as::<_, TitleWithAuthor>(sql)
        .fetch_all(&state.db)
        .await
    {
        Ok(twa) => HttpResponse::Ok().json(twa),
        Err(e) => {
            eprintln!("{e}");
            HttpResponse::NotFound().json("No title & author pairings found.")
        }
    }
}

#[get("/texts/{name}")]
pub async fn fetch_all_text_titles_by_author(
    state: Data<AppState>,
    path: Path<String>,
) -> impl Responder {
    let name = path.into_inner();
    println!("{name}");
    let sql = "SELECT texts.title, authors.first_name, authors.last_name FROM texts JOIN authors ON texts.author_id = authors.author_id WHERE authors.last_name = $1";
    match sqlx::query_as::<_, TitleWithAuthor>(sql)
        .bind(name)
        .fetch_all(&state.db)
        .await
    {
        Ok(twa) => HttpResponse::Ok().json(twa),
        Err(e) => {
            eprintln!("{e}");
            HttpResponse::NotFound().json("No title & author pairings found.")
        }
    }
}
