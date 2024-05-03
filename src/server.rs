use std::io::Write;
use std::net::TcpStream;

use super::request::Request;
use super::response::{HttpCode, Response};

pub fn handle_request(request: Request, stream: &mut TcpStream) {

    match request.get_path() {
        "/" => stream
            .write_all(Response::new_empty(HttpCode::OK).to_string().as_bytes())
            .unwrap(),
        "/user-agent" => stream
            .write_all(
                Response::new(
                    HttpCode::OK,
                    request.get_header("User-Agent"),
                    "text/plain",
                )
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
}