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

    
    /*
    This will provide the response to the request which looks like:

        Request: GET / HTTP/1.1
        Host: 127.0.0.1:7926
        Connection: keep-alive
        Cache-Control: max-age=0
        sec-ch-ua: "Chromium";v="110", "Not A(Brand";v="24", "Brave";v="110"
        sec-ch-ua-mobile: ?0
        sec-ch-ua-platform: "macOS"
        Upgrade-Insecure-Requests: 1
        User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36
    */

    stream.read(&mut buffer).unwrap();
    let response: &str = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
