use anyhow::Result;
use axum::{Router, routing::{get, post, put, delete}, Extension, Json, http::StatusCode, extract::{Path, self}};
use sqlx::SqlitePool;

use crate::db::{Book, all_books, book_by_id};


pub fn books_service() -> Router {
    Router::new()
        .route("/", get(get_all_books))
        .route("/:id", get(get_book))
        .route("/add", post(add_book))
        .route("/edit", put(update_book))
        .route("/delete/:id", delete(delete_book))
}


async fn get_all_books(
    Extension(connect): Extension<SqlitePool>
) -> Result<Json<Vec<Book>>, StatusCode> {
    if let Ok(books) = all_books(&connect).await {
        Ok(Json(books))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn get_book(
    Extension(connect): Extension<SqlitePool>,
    Path(id): Path<i32>
) -> Result<Json<Book>, StatusCode> {
    if let Ok(book) = book_by_id(&connect, id).await {
        Ok(Json(book))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn add_book(
    Extension(connect): Extension<SqlitePool>,
    extract::Json(book): extract::Json<Book>
) -> Result<Json<i32>, StatusCode> {
    if let Ok(new_id) = crate::db::add_book(&connect, &book.title, &book.author).await {
        Ok(Json(new_id))       
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn update_book(
    Extension(connect): Extension<SqlitePool>,
    extract::Json(book): extract::Json<Book>
) -> StatusCode {
    if crate::db::update_book(&connect, &book).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

async fn delete_book(
    Extension(connect): Extension<SqlitePool>,
    Path(id): Path<i32>
) -> StatusCode {
    if crate::db::delete_book(&connect, id).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}