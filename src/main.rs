use dotenv::dotenv;
use futures::executor::block_on;
use sea_orm::{Database, DbErr};
use std::env;

const DATABASE_NAME: &str = "ryans_test";

async fn run() -> Result<(), DbErr> {
    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    let db = Database::connect(&database_url).await?;

    Ok(())
}

fn main() {
    dotenv().ok();

    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
