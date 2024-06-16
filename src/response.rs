use core::str;
use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
pub enum ContentEncoding {
    Gzip,
}
impl ContentEncoding {
    fn to_string(&self) -> String {
        match self {
            ContentEncoding::Gzip => "gzip".to_string(),
        }
    }
    pub fn from_string(text :&String) -> Option<Self> {
        match text.to_ascii_lowercase().as_str() {
            "gzip" => Some(ContentEncoding::Gzip),
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum ContentType {
    PlainText(String),
    OctetStream(Vec<u8>),
}
impl ContentType {
    fn get_label(&self) -> &str {
        match self {
            ContentType::PlainText(_) => "text/plain",
            ContentType::OctetStream(_) => "application/octet-stream",
        }
    }
    fn to_string(&self) -> String {
        match self {
            ContentType::PlainText(content) => content.clone(),
            ContentType::OctetStream(content) => content.iter().map(|&byte| byte as char).collect(),
        }
    }
    fn len(&self) -> usize {
        match self {
            ContentType::PlainText(content) => content.len(),
            ContentType::OctetStream(content) => content.len(),
        }
    }
}
#[derive(Debug)]
pub enum HttpCode {
    OK = 200,
    Created = 201,
    BadRequest = 400,
    NotFound = 404,
}
impl HttpCode {
    pub fn to_string(&self) -> String {
        match self {
            HttpCode::OK => "200 OK",
            HttpCode::Created => "201 Created",
            HttpCode::BadRequest => "400 Bad Request",
            HttpCode::NotFound => "404 Not Found",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct Response {
    header: StartLine,
    http_headers: HashMap<String, String>,
    body: ContentType,
}
impl Response {
    pub fn to_string(&self) -> String {
        let mut fmt_headers = self
            .http_headers
            .iter()
            .map(|(key, header)| format!("{key}:{header}"));
        let head_str = fmt_headers.join("\r\n");

        println!("{:?}", self);
        format!(
            "{}\r\n{}\r\n\r\n{}",
            self.header.to_string(),
            head_str,
            self.body.to_string()
        )
    }
    pub fn new_empty(code: HttpCode) -> Self {
        Self {
            header: StartLine::new(code),
            http_headers: HashMap::new(),
            body: ContentType::PlainText("".to_string()),
        }
    }
    pub fn new(
        code: HttpCode,
        content: ContentType,
        content_encoding: Option<ContentEncoding>,
    ) -> Self {
        let mut headers = HashMap::new();
        let content_type_str = content.get_label();

        headers.insert("Content-Length".to_string(), content.len().to_string());
        headers.insert("Content-Type".to_string(), content_type_str.to_string());

        if content_encoding.is_some() {
            headers.insert(
                "Content-Encoding".to_string(),
                content_encoding.unwrap().to_string(),
            );
        }

        Self {
            header: StartLine::new(code),
            http_headers: headers,
            body: content,
        }
    }
}
#[derive(Debug)]
pub struct StartLine {
    protocol: String,
    http_code: HttpCode,
}

impl StartLine {
    pub fn to_string(&self) -> String {
        format!("{} {}", self.protocol, self.http_code.to_string())
    }
    pub fn new(code: HttpCode) -> Self {
        Self {
            protocol: PROTOCOL_VERSION.to_string(),
            http_code: code,
        }
    }
}
const PROTOCOL_VERSION: &str = "HTTP/1.1";
