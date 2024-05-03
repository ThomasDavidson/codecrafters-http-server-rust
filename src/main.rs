// Uncomment this block to pass the first stage
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

mod request;
use request::Request;
mod response;
use response::{HttpCode, Response};

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

                match request.get_path() {
                    "/" => stream
                        .write_all(Response::new_empty(HttpCode::OK).to_string().as_bytes())
                        .unwrap(),
                    "/user-agent" => stream
                        .write_all(
                            Response::new(HttpCode::OK, request.get_header("User-Agent"), "text/plain")
                                .to_string()
                                .as_bytes(),
                        )
                        .unwrap(),
                    header => {
                        if header.starts_with("/echo/") {
                            let body = &header[6..];
                            stream
                                .write_all(
                                    Response::new(HttpCode::OK, body, "text/plain")
                                        .to_string()
                                        .as_bytes(),
                                )
                                .unwrap()
                        } else {
                            stream
                                .write_all(
                                    Response::new_empty(HttpCode::NotFound)
                                        .to_string()
                                        .as_bytes(),
                                )
                                .unwrap()
                        }
                    }
                }

                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
