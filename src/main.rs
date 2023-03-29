#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome Page"
}

#[get("/api")]
fn api() -> &'static str {
    "Hello, world!"
}

#[get("/api/<name>")]
fn name(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, api, name])
}