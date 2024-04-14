
use std::{
    io::{
        prelude::*,
        BufReader,
    },
    net::{
        TcpListener,
        TcpStream,
        SocketAddr
    }
};

use crate::http::{request::Request, utils::DateTime};

mod http;

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Received request from {}", stream.peer_addr().unwrap());
    println!("Message: {:#?}", http_request);
    let request = Request::new(http_request);
    println!("Request: {:?}", request);


    let response = r#"HTTP/1.1 200 OK
Date: Sun, 14 Apr 2024 05:22:58 GMT
Content-Type: application/json
Content-Length: 13
Connection: keep-alive
Server: nginx/1.18.0 (Ubuntu)
Vary: Accept, Origin
Allow: OPTIONS, GET
X-Frame-Options: DENY
X-Content-Type-Options: nosniff
Referrer-Policy: same-origin
Cross-Origin-Opener-Policy: same-origin

{"test": 123}
"#;

    println!("Response is: ");
    println!("{}", response);
    let response = response.as_bytes();
    stream.write(response).expect("Faild to response to client");

}


fn main(){
    let addr = SocketAddr::from(([127, 0, 0, 1], 8069));
    let listener = TcpListener::bind(addr).expect("Faile to bind to address");
    println!("Server listening at address: {}", addr);
    let date = DateTime::now().unwrap();
    println!("DateTime: {:?}", date);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // spawn(|| handle_client(stream));
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}


