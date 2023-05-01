use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

use diesel::pg::PgConnection;
use diesel::Connection;

use dotenv::dotenv;
use std::env;

#[macro_use] extern crate rocket;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[get("/")]
fn index() -> &'static str {
    "Welcome Page"
}

#[get("/api")]
fn get_all_users() -> Json<User> {
   Json(
    User {
        id: 1,
        name: "Ryan".to_string(),
        email: "test@email.com".to_string(),
    }
   )
}

#[get("/api/get-user/<id>")]
fn id(id: i32) -> Json<User> {
    Json(
        User {
            id,
            name: "Ryan".to_string(),
            email: "test@email.com".to_string(),
        }
    )
}

#[post("/api/add-user")]
fn add_user() -> Json<User> {
    Json(
        User {
            id: 12,
            name: "Ryan".to_string(),
            email: "test@email.com".to_string()
        }
    )
}

#[put("/api/update-user")]
fn update_user() -> &'static str {
    "Update User - Todo"
}

#[delete("/api/delete-user")]
fn delete_user() -> &'static str {
    "Delete User - Todo"
}

#[launch]
fn rocket() -> _ {
    establish_connection();
    rocket::build()
        .mount("/", routes![index, get_all_users, id, add_user, update_user, delete_user])
}