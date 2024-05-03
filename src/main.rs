// Uncomment this block to pass the first stage
use core::str;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

mod response;
use response::{HttpCode, Response};

#[derive(Debug)]
enum Method {
    GET,
    POST,
}
impl Method {
    fn from_string(word: &str) -> Option<Self> {
        match word {
            "GET" => Some(Self::GET),
            "POST" => Some(Self::POST),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Header {
    method: Method,
    path: String,
    protocol: String,
}

impl Header {
    fn from_string(header_data: &str) -> Option<Self> {
        let mut header_split = header_data.split(" ");

        let Some(method_str) = header_split.next() else {
            println!("method_str");
            return None;
        };

        let Some(method) = Method::from_string(method_str) else {
            println!("method {method_str}");
            return None;
        };

        let Some(path) = header_split.next() else {
            println!("path");
            return None;
        };

        let Some(protocol) = header_split.next() else {
            println!("protocol");
            return None;
        };
        Some(Self {
            method: method,
            path: path.to_string(),
            protocol: protocol.to_string(),
        })
    }
}

#[derive(Debug)]
struct Request {
    request_line: Header,
    http_headers: HashMap<String, String>,
    body: String,
}
impl Request {
    fn new(mut stream: &TcpStream) -> Option<Self> {
        let buf_reader = BufReader::new(&mut stream);

        let mut request = buf_reader
            .lines()
            .map(|l| l.unwrap())
            .take_while(|line| !line.is_empty());

        let header_str = request.next().unwrap();
        let Some(header) = Header::from_string(header_str.as_str()) else {
            return None;
        };
        let mut http_headers: HashMap<String, String> = HashMap::new();

        while let Some(line) = request.next() {
            if line.as_str() == "" {
                break;
            }
            let Some((key, value)) = line.split_once(":")else {
                panic!("Cannot split {:?}", line);
            };
            http_headers.insert(key.to_string(), value.to_string());
        }

        Some(Request {
            request_line: header,
            http_headers: http_headers,
            body: request.collect(),
        })
    }
}

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
                            Response::new_empty(HttpCode::BadRequest, "")
                                .to_string()
                                .as_bytes(),
                        )
                        .unwrap();
                    continue;
                };

                match request.request_line.path.as_str() {
                    "/" => stream
                        .write_all(Response::new_empty(HttpCode::OK, "").to_string().as_bytes())
                        .unwrap(),
                    "/user-agent" => stream
                        .write_all(
                            Response::new(HttpCode::OK, request.http_headers.get("User-Agent").unwrap(), "text/plain")
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
                                    Response::new_empty(HttpCode::NotFound, "")
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
