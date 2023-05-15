use postgres::{ Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener, TcpStream };
use std::io::{Read, Write};
use std::env;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String
}

// const DB_URL: &str = env!("DATABASE_URL");

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 Internal Server Error\r\n";

fn main() {
    println!("Hello");
}
