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
    print!("Request: {}", String::from_utf8_lossy(&buffer[..]))
}
