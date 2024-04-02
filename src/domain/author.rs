use std::error::Error;

use actix_web::{
    get,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, prelude::FromRow, query, PgPool, Row};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Author {
    pub author_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
}

impl Author {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self {
            author_id: None,
            first_name,
            last_name,
        }
    }

    pub fn row_to_author(row: PgRow) -> Self {
        Self {
            author_id: row.get("author_id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
        }
    }

    pub async fn create(state: Data<AppState>, author: &Author) -> impl Responder {
        let sql = "INSERT INTO authors (first_name, last_name) VALUES ($1, $2)";

        match sqlx::query_as::<_, Author>(sql)
            .bind(&author.first_name)
            .bind(&author.last_name)
            .fetch_one(&state.db)
            .await
        {
            Ok(au) => HttpResponse::Ok().json(au),
            Err(_) => HttpResponse::InternalServerError().json("Failed to create new author"),
        }
    }
}
