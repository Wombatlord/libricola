use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    domain::{author::Author, text::Text},
    AppState,
};

#[post("/create/author")]
pub async fn create_author(state: Data<AppState>, author: Json<Author>) -> impl Responder {
    // curl -H 'Content-Type: application/json'
    //      -d '{"first_name": "Emily", "last_name":"Bronte"}'
    //      -X POST http://localhost:8080/create/author

    let sql = "INSERT INTO authors (first_name, last_name) VALUES ($1, $2) RETURNING first_name, last_name";
    println!("YOU RANG?");
    match sqlx::query_as::<_, Author>(sql)
        .bind(&author.first_name)
        .bind(&author.last_name)
        .fetch_one(&state.db)
        .await
    {
        Ok(author) => {
            println!("{author:?}");
            HttpResponse::Ok().json(author)
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to create new author"),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateText {
    pub text: Text,
    pub author: Author,
}

#[post("/create/text")]
pub async fn create_text(state: Data<AppState>, create_text: Json<CreateText>) -> impl Responder {
    // curl -H 'Content-Type: application/json' -d '[{"text_type_id": 1, "author_id": 0, "title": "Wuthering Heights", "published": 1847, "metadata": {"genre_tags": ["Gothic"]}}, {"first_name": "Emily", "last_name":"Bronte"}]' -X POST http://localhost:8080/create/text

    println!("I'm tryin' chief");
    let sql = "SELECT author_id, first_name, last_name FROM authors WHERE authors.first_name = $1";
    println!("{create_text:?}");

    let Ok(mut txn) = state.db.begin().await else {
        return HttpResponse::InternalServerError().json("Failed to create transaction");
    };

    if let Ok(Some(author)) = sqlx::query_as::<_, Author>(sql)
        .bind(&create_text.author.first_name)
        .bind(&create_text.author.last_name)
        .fetch_optional(&state.db)
        .await
    {
        let author_id = author.author_id.unwrap();

        let text_sql = "INSERT INTO texts (text_type_id, author_id, title, published, metadata) VALUES ($1, $2, $3, $4, $5) RETURNING text_type_id, author_id, title, published, metadata";

        let ser = json!(&create_text.text.metadata);
        match sqlx::query_as::<_, Text>(text_sql)
            .bind(&create_text.text.text_type_id)
            .bind(&author_id)
            .bind(&create_text.text.title)
            .bind(&create_text.text.published)
            .bind(&ser)
            .fetch_one(&mut *txn)
            .await
        {
            Ok(text) => {
                let Ok(_) = txn.commit().await else {
                    return HttpResponse::InternalServerError()
                        .json("Failed to commit transaction");
                };
                HttpResponse::Ok().json(text)
            }
            Err(e) => {
                eprintln!("{e}");
                HttpResponse::InternalServerError().json("Failed to create new text")
            }
        }
    } else {
        let author_sql = "INSERT INTO authors (first_name, last_name) VALUES ($1, $2) RETURNING authors.author_id, authors.first_name, authors.last_name";
        match sqlx::query_as::<_, Author>(author_sql)
            .bind(&create_text.author.first_name)
            .bind(&create_text.author.last_name)
            .fetch_one(&mut *txn)
            .await
        {
            Ok(author) => {
                let text_sql = "INSERT INTO texts (text_type_id, author_id, title, published, metadata) VALUES ($1, $2, $3, $4, $5) RETURNING text_type_id, author_id, title, published, metadata";

                let ser = json!(&create_text.text.metadata);
                match sqlx::query_as::<_, Text>(text_sql)
                    .bind(&create_text.text.text_type_id)
                    .bind(&author.author_id)
                    .bind(&create_text.text.title)
                    .bind(&create_text.text.published)
                    .bind(&ser)
                    .fetch_one(&mut *txn)
                    .await
                {
                    Ok(text) => {
                        let Ok(_) = txn.commit().await else {
                            return HttpResponse::InternalServerError()
                                .json("Failed to commit transaction");
                        };
                        return HttpResponse::Ok().json(text);
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        return HttpResponse::InternalServerError()
                            .json("Failed to create new text");
                    }
                }
            }
            Err(e) => {
                eprintln!("{e}");
                return HttpResponse::InternalServerError().json("Failed to create new author");
            }
        }
    }
}