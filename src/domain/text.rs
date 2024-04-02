use actix_web::{web::Json, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, Postgres, Transaction};

use super::{author::Author, request_objects::CreateTextRequest, text_types::TextType};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Metadata {
    pub genre_tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Text {
    pub text_type_id: i32,
    pub author_id: i32,
    pub title: String,
    pub published: i32,
    #[sqlx(json)]
    pub metadata: Metadata,
}

impl Text {
    #[allow(dead_code)]
    pub fn new(
        text_type_id: i32,
        author_id: i32,
        title: String,
        published: i32,
        metadata: Metadata,
    ) -> Self {
        Self {
            text_type_id,
            author_id,
            title,
            published,
            metadata,
        }
    }

    pub async fn with_extant_author(
        mut txn: Transaction<'_, Postgres>,
        author: Author,
        create_text: Json<CreateTextRequest>,
        text_type: TextType,
    ) -> HttpResponse {
        let author_id = author.author_id.unwrap();

        let text_sql = "INSERT INTO texts (text_type_id, author_id, title, published, metadata) VALUES ($1, $2, $3, $4, $5) RETURNING text_type_id, author_id, title, published, metadata";

        let ser = json!(&create_text.text.metadata);
        let text = match sqlx::query_as::<_, Text>(text_sql)
            .bind(&text_type.text_type_id)
            .bind(&author_id)
            .bind(&create_text.text.title)
            .bind(&create_text.text.published)
            .bind(&ser)
            .fetch_one(&mut *txn)
            .await
        {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{e}");
                return HttpResponse::InternalServerError()
                    .json("Failed to insert new text with Error: {e}");
            }
        };

        match txn.commit().await {
            Ok(_) => HttpResponse::Ok().json(text),
            Err(e) => HttpResponse::InternalServerError()
                .json(format!("Failed to commit transaction with Error: {e}")),
        }
    }

    pub async fn with_new_author(
        mut txn: Transaction<'_, Postgres>,
        create_text_and_author: Json<CreateTextRequest>,
        text_type: TextType,
    ) -> HttpResponse {
        let author_sql = "INSERT INTO authors (first_name, last_name) VALUES ($1, $2) RETURNING authors.author_id, authors.first_name, authors.last_name";
        let author = match sqlx::query_as::<_, Author>(author_sql)
            .bind(&create_text_and_author.author.first_name)
            .bind(&create_text_and_author.author.last_name)
            .fetch_one(&mut *txn)
            .await
        {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{e}");
                return HttpResponse::InternalServerError()
                    .json("Failed to insert new author with Error: {e}");
            }
        };

        let text_sql = "INSERT INTO texts (text_type_id, author_id, title, published, metadata) VALUES ($1, $2, $3, $4, $5) RETURNING text_type_id, author_id, title, published, metadata";
        let json_metadata = json!(&create_text_and_author.text.metadata);

        let text = match sqlx::query_as::<_, Text>(text_sql)
            .bind(&text_type.text_type_id)
            .bind(&author.author_id)
            .bind(&create_text_and_author.text.title)
            .bind(&create_text_and_author.text.published)
            .bind(&json_metadata)
            .fetch_one(&mut *txn)
            .await
        {
            Ok(text) => text,
            Err(e) => {
                eprintln!("{e}");
                return HttpResponse::InternalServerError()
                    .json(format!("Failed to insert new text with Error: {e}"));
            }
        };

        match txn.commit().await {
            Ok(_) => HttpResponse::Ok().json(text),
            Err(e) => HttpResponse::InternalServerError()
                .json(format!("Failed to commit transaction with Error: {e}")),
        }
    }
}
