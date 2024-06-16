use core::str;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::vec;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
}
impl Method {
    pub fn from_string(word: &str) -> Option<Self> {
        match word {
            "GET" => Some(Self::GET),
            "POST" => Some(Self::POST),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Header {
    method: Method,
    path: String,
    protocol: String,
}

impl Header {
    pub fn from_string(header_data: &str) -> Option<Self> {
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
pub struct Request {
    request_line: Header,
    http_headers: HashMap<String, String>,
    pub body: Vec<u8>,
}
impl Request {
    pub fn new(mut stream: &TcpStream) -> Option<Self> {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut head = String::new();

        while let Ok(r) = buf_reader.read_line(&mut head) {
            if r < 3 {
                //detect empty line
                break;
            }
        }
        let mut request = head.lines();

        let header_str = request.next().unwrap();
        let Some(header) = Header::from_string(header_str) else {
            return None;
        };
        let mut http_headers: HashMap<String, String> = HashMap::new();

        while let Some(line) = request.next() {
            let Some((key, value)) = line.split_once(": ") else {
                continue;
            };
            http_headers.insert(key.to_string(), value.to_string());
        }

        let body: Vec<u8> = match http_headers.get("Content-Length") {
            Some(a) => {
                match a.parse::<usize>() {
                    Ok(size) => {
                        let mut buffer = vec![0; size]; //New Vector with size of Content
                        buf_reader.read_exact(&mut buffer).unwrap(); //Get the Body Content.
                        buffer
                    }
                    Err(e) => {
                        println!("Body Parse Error: {:?}", e);
                        vec![]
                    },
                }
            }
            None => vec![],
        };
        Some(Request {
            request_line: header,
            http_headers,
            body,
        })
    }
    pub fn get_header(&self, header: &str) -> Option<&String> {
        self.http_headers.get(header)
    }

    pub fn get_path(&self) -> &str {
        &self.request_line.path
    }
    pub fn get_method(&self) -> &Method {
        &self.request_line.method
    }
}
