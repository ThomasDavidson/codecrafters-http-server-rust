// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::{io::Write, thread};

mod request;
use request::Request;
mod response;
use response::{HttpCode, Response};

use crate::server::handle_request;
mod server;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let Some(request) = Request::new(&mut stream) else {
                        stream
                            .write_all(
                                Response::new_empty(HttpCode::BadRequest)
                                    .to_string()
                                    .unwrap()
                                    .as_bytes(),
                            )
                            .unwrap();
                        return;
                    };
                    handle_request(request, &mut stream);
                });

                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
