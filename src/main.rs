// Uncomment this block to pass the first stage
use core::str;
use std::io::{Read, Write};
use std::net::TcpListener;

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

enum HttpCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}
impl HttpCode {
    fn to_string(&self) -> String {
        match self {
            HttpCode::OK => "200 OK",
            HttpCode::BadRequest => "400 Bad Request",
            HttpCode::NotFound => "404 Not found",
        }
        .to_string()
    }
}
struct Response {
    protocol: String,
    http_code: HttpCode,
}
impl Response {
    fn to_string(&self) -> String {
        format!("{} {}\r\n\r\n", self.protocol, self.http_code.to_string())
    }
    fn new(code: HttpCode) -> Response {
        Self {
            protocol: PROTOCOL_VERSION.to_string(),
            http_code: code,
        }
    }
}
const PROTOCOL_VERSION: &str = "HTTP/1.1";

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

                let mut buf: [u8; 256] = [0; 256];
                let Ok(size) = stream.read(&mut buf) else {
                    break;
                };
                let Ok(str_resp) = str::from_utf8(&buf[0..size]) else {
                    break;
                };

                let resp_lines = str_resp.lines().collect::<Vec<&str>>();

                let Some(header) = Header::from_string(resp_lines[0]) else {
                    stream.write_all(Response::new(HttpCode::BadRequest).to_string().as_bytes()).unwrap();
                    continue;
                };

                match header.path.as_str() {
                    "/" => stream.write_all(Response::new(HttpCode::OK).to_string().as_bytes()).unwrap(),
                    _ => stream.write_all(Response::new(HttpCode::NotFound).to_string().as_bytes()).unwrap(),
                }

                println!("{:?}", header);
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
