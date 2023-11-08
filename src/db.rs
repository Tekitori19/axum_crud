use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Row, SqlitePool};
use anyhow::{Result, Ok};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Book {
    //ID cua sach
    pub id: i32,
    //Title cua sach
    pub title: String,
    //Author cua sach
    pub author: String,
} 

pub async fn all_books(connection_pool: &SqlitePool) -> Result<Vec<Book>> {
    Ok(
        sqlx::query_as::<_, Book>("SELECT * FROM books ORDER BY title, author")
            .fetch_all(connection_pool)
            .await?
    )
}

pub async fn book_by_id(connection_pool: &SqlitePool, id: i32) -> Result<Book> {
    Ok(
        sqlx::query_as::<_,Book>("SELECT * FROM books WHERE id=$1")
            .bind(id)
            .fetch_one(connection_pool)
            .await?
    )
} 

pub async fn add_book<S: ToString>(
    connection_pool: &SqlitePool,
    title: S,
    author: S
) -> Result<i32> {
    let title = title.to_string();
    let author = author.to_string();
    Ok(
        sqlx::query("INSERT INTO books (title, author) VALUES ($1, $2) RETURNING id")
            .bind(title)
            .bind(author)
            .fetch_one(connection_pool)
            .await?
            .get(0)
    )
}

pub async fn update_book(connection_pool: &SqlitePool, book: &Book) -> Result<()> {
    sqlx::query("UPDATE books SET title=$1, author=$2 WHERE id=$3")
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.id)
        .execute(connection_pool)
        .await?;
    Ok(())
}

pub async fn delete_book(connection_pool: &SqlitePool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM books WHERE id=$1")
        .bind(id)
        .execute(connection_pool)
        .await?;
    Ok(())
}

pub async fn init_db() -> Result<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL")?;
    let connection_pool = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!().run(&connection_pool).await?;
    Ok(connection_pool)
}