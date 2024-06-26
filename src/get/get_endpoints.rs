use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};

use crate::{
    domain::{author::Author, request_objects::TitleWithAuthor, text_types::TextType},
    get::get_helpers::{full_name_search, last_name_search},
    AppState,
};

#[get("/authors")]
pub async fn fetch_all_authors(state: Data<AppState>) -> impl Responder {
    let sql = "SELECT author_id, first_name, last_name FROM authors";
    match sqlx::query_as::<_, Author>(sql).fetch_all(&state.db).await {
        Ok(authors) => HttpResponse::Ok().json(authors),
        Err(e) => {
            eprintln!("{e}");
            HttpResponse::NotFound().json("No authors found.")
        }
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
    let sql = "SELECT texts.title, authors.first_name, authors.last_name, texts.published FROM texts JOIN authors ON texts.author_id = authors.author_id";
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
    let length_check: Vec<&str> = name.split(" ").collect();
    if length_check.len() == 1 {
        return last_name_search(name, state).await;
    } else {
        return full_name_search(name, state).await;
    }
}
