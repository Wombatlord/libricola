use actix_web::{web::Data, HttpResponse};
use serde_json::{Map, Value};

use crate::{domain::request_objects::TitleWithAuthor, AppState};

pub fn split_name_to_first_and_last(name: String) -> (String, String) {
    let split: Vec<&str> = name.split(" ").collect();
    let first = split[0].to_string();
    let last = split.last().unwrap().to_string();
    (first, last)
}

pub async fn last_name_search(last_name: String, executor: Data<AppState>) -> HttpResponse {
    let sql = "SELECT texts.title, authors.first_name, authors.last_name, texts.published FROM texts JOIN authors ON texts.author_id = authors.author_id WHERE authors.last_name ILIKE $1";
    match sqlx::query_as::<_, TitleWithAuthor>(sql)
        .bind(last_name)
        .fetch_all(&executor.db)
        .await
    {
        Ok(twa) => {
            if twa.len() == 0 {
                return HttpResponse::NotFound().json("No title & author pairings found.");
            };

            let response = all_text_titles_by_author_mapped(twa);

            println!("{response:?}");
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("{e}");
            HttpResponse::NotFound().json("No title & author pairings found.")
        }
    }
}

pub async fn full_name_search(full_name: String, executor: Data<AppState>) -> HttpResponse {
    let split = split_name_to_first_and_last(full_name);
    println!("{split:?}");
    let sql = "SELECT texts.title, authors.first_name, authors.last_name, texts.published FROM texts JOIN authors ON texts.author_id = authors.author_id WHERE authors.first_name ILIKE $1 AND authors.last_name ILIKE $2";
    match sqlx::query_as::<_, TitleWithAuthor>(sql)
        .bind(split.0 + "%")
        .bind(split.1)
        .fetch_all(&executor.db)
        .await
    {
        Ok(twa) => {
            if twa.len() == 0 {
                return HttpResponse::NotFound().json("No title & author pairings found.");
            };
            let response = all_text_titles_by_author_mapped(twa);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("{e}");
            HttpResponse::NotFound().json("No title & author pairings found.")
        }
    }
}

pub fn all_text_titles_by_author_mapped(
    text_with_author: Vec<TitleWithAuthor>,
) -> Map<String, Value> {
    let mut main_map = Map::new();
    let mut titles_map = Map::new();
    let mut name = "".to_string();
    for entry in text_with_author {
        if name != format!("{} {}", entry.first_name, entry.last_name) {
            titles_map.clear();
        }
        name = format!("{} {}", entry.first_name, entry.last_name);
        titles_map.insert(entry.title, entry.published.into());
        main_map.insert(name.clone(), Value::Object(titles_map.clone()));
    }
    main_map
}
