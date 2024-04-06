
use std::{
    // thread::spawn,
    io::{
        prelude::*,
        stdin,
    },
    net::{
        TcpListener,
        TcpStream,
        SocketAddr
    }
};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 2048];

    stream.read(&mut buffer).expect("Faild to read from client");

    let request = String::from_utf8(buffer.to_vec()).expect("Fail to decode received data");
    println!("Received request from {}", stream.peer_addr().unwrap());
    println!("Message: {}", request);

    let mut response = String::new();
    println!("Enter response:");
    stdin().read_line(&mut response).unwrap();

    let response = response.as_bytes();
    stream.write(response).expect("Faild to response to client");

}


fn main(){
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
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


