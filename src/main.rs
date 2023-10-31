mod db;
use crate::db::init_db;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    //load file .env
    dotenv::dotenv().ok();

    //Khoi tao database 
    let connection_pool = init_db().await?;
    
    Ok(())
}