use core::str;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

#[derive(Debug)]
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
    body: String,
}
impl Request {
    pub fn new(mut stream: &TcpStream) -> Option<Self> {
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
            let Some((key, value)) = line.split_once(": ") else {
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
    pub fn get_header(&self, header: &str) -> &str {
        self.http_headers.get(header).unwrap()
    }

    pub fn get_path(&self) -> &str {
        &self.request_line.path
    }
}
