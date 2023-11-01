mod db;
mod rest;
mod view;
use std::net::SocketAddr;

use crate::db::init_db;
use anyhow::Result;
use sqlx::SqlitePool;
use axum::{Router, Extension}; 

#[tokio::main]
async fn main() -> Result<()> {
    //load file .env
    dotenv::dotenv().ok();

    //Khoi tao database 
    let connection_pool = init_db().await?;

    let app = router(connection_pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

fn router(connection_pool: SqlitePool) -> Router {
    Router::new()
        .nest_service("/books", rest::books_service())
        .nest_service("/", view::view_service())
        .layer(Extension(connection_pool))
}