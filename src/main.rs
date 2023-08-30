extern crate dotenv;
use dotenv::dotenv;
use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbErr, Statement};

#[macro_use]
extern crate dotenv_codegen;

const DATABASE_URL: &str = dotenv!("DATABASE_URL");
const DATABASE_NAME: &str = dotenv!("DATABASE_NAME");

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("CREATE DATABASE \"{}\";", DATABASE_NAME),
    ))
    .await?;

    Ok(())
}

fn main() {
    dotenv().ok();

    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
