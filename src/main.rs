use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
}

#[get("/")]
fn index() -> &'static str {
    "Welcome Page"
}

#[get("/api")]
fn get_all_users() -> Json<User> {
   Json(
    User {
        name: "Ryan".to_string(),
    }
   )
}

#[get("/api/<name>")]
fn name(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[post("/api/add-user")]
fn add_user() -> &'static str {
    "Add User - Todo"
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
    rocket::build()
        .mount("/", routes![index, get_all_users, name, add_user, update_user, delete_user])
}