use core::str;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::collections::HashMap;
use std::io;
use std::io::Write;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum ContentEncoding {
    Gzip,
}
impl ContentEncoding {
    fn to_string(&self) -> String {
        match self {
            ContentEncoding::Gzip => "gzip".to_string(),
        }
    }
    pub fn from_string(text: &String) -> Vec<Self> {
        text.split(", ")
            .map(|t| match t.to_ascii_lowercase().as_str() {
                "gzip" => Some(ContentEncoding::Gzip),
                _ => None,
            })
            .filter(|t| t.is_some())
            .map(|t| t.unwrap())
            .collect()
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
    gzip_supported: bool,
}
impl Response {
    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {

        let mut bytes: Vec<u8> = Vec::new();


        let mut headers = self.http_headers.clone();

        let mut formatted_body = match self.gzip_supported {
            false => self.body.to_string().as_bytes().to_vec(),
            true => {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(self.body.to_string().as_bytes())?;
                encoder.finish()?
            }
        };

        headers.insert(
            "Content-Length".to_string(),
            formatted_body.len().to_string(),
        );

        let mut fmt_headers = headers
            .iter()
            .map(|(key, header)| format!("{key}:{header}"));
        let head_str = fmt_headers.join("\r\n");

        let header = format!(
            "{}\r\n{}\r\n\r\n",
            self.header.to_string(),
            head_str,
        );

        bytes.append(&mut header.into_bytes());

        bytes.append(&mut formatted_body);

        Ok(bytes)
    }
    pub fn new_empty(code: HttpCode) -> Self {
        Self {
            header: StartLine::new(code),
            http_headers: HashMap::new(),
            body: ContentType::PlainText("".to_string()),
            gzip_supported: false,
        }
    }
    pub fn new(
        code: HttpCode,
        content: ContentType,
        content_encoding: Option<ContentEncoding>,
    ) -> Self {
        let mut headers = HashMap::new();
        let content_type_str = content.get_label();

        headers.insert("Content-Type".to_string(), content_type_str.to_string());

        let gzip_supported = match content_encoding {
            Some(ContentEncoding::Gzip) => true,
            _ => false,
        };

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
            gzip_supported,
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
