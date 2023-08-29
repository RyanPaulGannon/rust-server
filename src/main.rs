use dotenv::dotenv;
use futures::executor::block_on;
use sea_orm::{Database, DbErr};

#[macro_use]
extern crate dotenv_codegen;

const DATABASE_URL: &str = dotenv!("DATABASE_URL");
const DATABASE_NAME: &str = dotenv!("DATABASE_NAME");

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    println!("Connected to, {:?}", db);

    Ok(())
}

fn main() {
    dotenv().ok();

    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
