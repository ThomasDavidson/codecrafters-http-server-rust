use std::io::Write;
use std::net::TcpStream;
use std::{env, fs};

use super::request::Request;
use super::response::{ContentType, HttpCode, Response};

fn get_directory() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return None;
    }
    if &args[1] != "--directory" {
        return None;
    }

    println!("args: {:?}", args);
    let dir = &args[2];
    Some(dir.clone())
}

pub fn handle_request(request: Request, stream: &mut TcpStream) {
    let res = match request.get_path() {
        "/" => stream.write_all(Response::new_empty(HttpCode::OK).to_string().as_bytes()),
        "/user-agent" => stream.write_all(
            Response::new(
                HttpCode::OK,
                ContentType::PlainText(request.get_header("User-Agent").to_string()),
            )
            .to_string()
            .as_bytes(),
        ),
        header => {
            if header.starts_with("/echo/") {
                let body = &header[6..];
                stream.write_all(
                    Response::new(HttpCode::OK, ContentType::PlainText(body.to_string()))
                        .to_string()
                        .as_bytes(),
                )
            } else if header.starts_with("/files/") {
                let file_res = match get_directory() {
                    None => None,
                    Some(dir) => {
                        let filename = &header[7..];
                        let file_path = dir + filename;

                        match fs::read(file_path) {
                            Ok(file) => Some(file),
                            Err(_) => None,
                        }
                    }
                };

                match file_res {
                    None => stream.write_all(
                        Response::new_empty(HttpCode::NotFound)
                            .to_string()
                            .as_bytes(),
                    ),
                    Some(file) => stream.write_all(
                        Response::new(HttpCode::OK, ContentType::OctetStrean(file))
                            .to_string()
                            .as_bytes(),
                    ),
                }
            } else {
                stream.write_all(
                    Response::new_empty(HttpCode::NotFound)
                        .to_string()
                        .as_bytes(),
                )
            }
        }
    };
    println!("Result: {:?}", res);
}
