use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::*;

// TCP: Transport layer protocol which describes how data is transferred from one node to another
// HTTP: Application level protocol which describes how the data is structured

fn main() {
    // :ryan when typed on a phone
    let listener = TcpListener::bind("127.0.0.1:7926").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("index.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
