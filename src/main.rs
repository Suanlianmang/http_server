
use std::{
    io::{
        prelude::*,
        stdin,
        BufReader,
    },
    net::{
        TcpListener,
        TcpStream,
        SocketAddr
    }
};

use crate::message::requests::Requests;

mod message;

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Received request from {}", stream.peer_addr().unwrap());
    println!("Message: {:#?}", http_request);
    let request = Requests::new(http_request);
    println!("Request: {:?}", request);


    let response = r"HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 13

Hello, World!
        ";

    println!("Response is: ");
    println!("{}", response);
    let response = response.as_bytes();
    stream.write(response).expect("Faild to response to client");

}


fn main(){
    let addr = SocketAddr::from(([127, 0, 0, 1], 8069));
    let listener = TcpListener::bind(addr).expect("Faile to bind to address");
    println!("Server listening at address: {}", addr);

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


