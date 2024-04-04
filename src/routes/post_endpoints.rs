use crate::{
    app_state::AppState, domain::{
        author::Author, request_objects::CreateTextRequest, text::Text, text_types::TextType,
    }
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
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

#[post("/create/text")]
pub async fn create_text(
    state: Data<AppState>,
    create_text: Json<CreateTextRequest>,
) -> impl Responder {
    // curl -H 'Content-Type: application/json'
    // -d '[
    //  {"text_type_id": 1,
    //  "author_id": 0,
    //  "title": "Wuthering Heights",
    //  "published": 1847,
    //  "metadata": {"genre_tags": ["Gothic"]}
    //  },
    //  {"first_name": "Emily", "last_name":"Bronte"}
    //  ]'
    // -X POST http://localhost:8080/create/text

    println!("I'm tryin' chief");
    let sql = "SELECT author_id, first_name, last_name FROM authors WHERE authors.first_name = $1 AND authors.last_name = $2";
    println!("{create_text:?}");

    let Ok(txn) = state.db.begin().await else {
        return HttpResponse::InternalServerError().json("Failed to create transaction");
    };

    let text_type = match sqlx::query_as::<_, TextType>(
        "SELECT text_type_id, text_type FROM text_types WHERE text_type = $1",
    )
    .bind(&create_text.text.text_type)
    .fetch_one(&state.db)
    .await
    {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{e}");
            return HttpResponse::InternalServerError().json("Failed to find matching text type.");
        }
    };

    if let Ok(Some(author)) = sqlx::query_as::<_, Author>(sql)
        .bind(&create_text.author.first_name)
        .bind(&create_text.author.last_name)
        .fetch_optional(&state.db)
        .await
    {
        let response = Text::with_extant_author(txn, author, create_text, text_type).await;
        response
    } else {
        let response = Text::with_new_author(txn, create_text, text_type).await;
        response
    }
}
