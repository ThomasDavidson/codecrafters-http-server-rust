// Uncomment this block to pass the first stage
use std::io::Write;
use std::net::TcpListener;

mod request;
use request::Request;
mod response;
use response::{HttpCode, Response};

use crate::server::handle_request;
mod server;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // let response = "HTTP/1.1 200 OK\r\n\r\n";

                let Some(request) = Request::new(&mut stream) else {
                    stream
                        .write_all(
                            Response::new_empty(HttpCode::BadRequest)
                                .to_string()
                                .as_bytes(),
                        )
                        .unwrap();
                    continue;
                };

                let mut stream_clone = stream.try_clone().expect("clone failed...");
                let _ = handle_request(request, &mut stream_clone);

                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
