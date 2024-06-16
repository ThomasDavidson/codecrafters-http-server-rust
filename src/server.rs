use std::io::Write;
use std::net::TcpStream;
use std::{env, fs};

use crate::request::Method;

use super::request::Request;
use super::response::{ContentEncoding, ContentType, HttpCode, Response};

fn get_directory() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return None;
    }
    if &args[1] != "--directory" {
        return None;
    }

    let dir = &args[2];
    Some(dir.clone())
}

pub fn handle_request(request: Request, stream: &mut TcpStream) {
    let accepted_encoding_str = request.get_header("Accept-Encoding");
    let accepted_encoding = match accepted_encoding_str {
        None => Vec::new(),
        Some(s) => ContentEncoding::from_string(s),
    };
    println!("{:?}", accepted_encoding);

    let user_agent = request.get_header("User-Agent");

    let res = match (request.get_path(), request.get_method()) {
        ("/", _) => stream.write_all(Response::new_empty(HttpCode::OK).to_string().as_bytes()),
        ("/user-agent", _) => stream.write_all(
            Response::new(
                HttpCode::OK,
                ContentType::PlainText(user_agent.unwrap().clone()),
                None,
            )
            .to_string()
            .as_bytes(),
        ),
        (header, method) => {
            if header.starts_with("/echo/") {
                let body = &header[6..];
                let gzip_support = match accepted_encoding.contains(&ContentEncoding::Gzip) {
                    false => None,
                    true => Some(ContentEncoding::Gzip),
                };
                stream.write_all(
                    Response::new(
                        HttpCode::OK,
                        ContentType::PlainText(body.to_string()),
                        gzip_support,
                    )
                    .to_string()
                    .as_bytes(),
                )
            } else if header.starts_with("/files/") && *method == Method::GET {
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
                        Response::new(HttpCode::OK, ContentType::OctetStream(file), None)
                            .to_string()
                            .as_bytes(),
                    ),
                }
            } else if header.starts_with("/files/") && *method == Method::POST {
                let file_res = match get_directory() {
                    None => None,
                    Some(dir) => {
                        let filename = &header[7..];
                        let file_path = dir + filename;

                        match fs::write(file_path, &request.body) {
                            Ok(file) => Some(file),
                            Err(e) => {
                                println!("File write error: {:?}", e);
                                None
                            }
                        }
                    }
                };
                match file_res {
                    None => stream.write_all(
                        Response::new_empty(HttpCode::NotFound)
                            .to_string()
                            .as_bytes(),
                    ),
                    Some(_) => stream.write_all(
                        Response::new_empty(HttpCode::Created)
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
}
