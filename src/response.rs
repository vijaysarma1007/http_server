use anyhow::{Context, Ok, Result};
use std::{fmt::Display, io::Write, net::TcpStream};

use crate::request::Request;

#[derive(Debug)]
pub enum HttpCode {
    Ok,
    NotFound,
    YourFault,
    Created,
}

impl Display for HttpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (number, message) = match self {
            Self::Ok => (200, "OK"),
            Self::NotFound => (404, "Not Found"),
            Self::YourFault => (400, "Bad Request"),
            Self::Created => (201, "Created"),
        };

        write!(f, "{number} {message}")
    }
}

#[derive(Debug)]
pub enum ContentType {
    TextPlain,
    ApplicationOctetStream,
}

#[derive(Debug)]
pub struct Response {
    pub code: HttpCode,
    pub body: Option<String>,
    pub content_type: ContentType,
}

impl Response {
    #[allow(dead_code)]
    pub fn build(&self) -> Result<String> {
        let code = &self.code;
        let body = self.body.clone().unwrap_or_default();
        let response = format!("HTTP/1.1 {code}\r\n\r\n{body}");
        Ok(response)
    }

    fn content_type_header(&self) -> String {
        let raw_header = match self.content_type {
            ContentType::TextPlain => "text/plain",
            ContentType::ApplicationOctetStream => "application/octet-stream",
        };

        format!("Content-Type: {raw_header}\r\n")
    }

    fn content_length_header(&self) -> Option<String> {
        let Some(body) = self.body.as_ref() else {
            return None;
        };

        let length = body.as_bytes().len();
        Some(format!("Content-Length: {length}\r\n"))
    }
}

pub fn send_response(stream: &mut TcpStream, response: Response, request: &Request) -> Result<()> {
    //writing protocol
    write!(stream, "HTTP/1.1 ").context("writing protocol")?;
    //writing response code
    write!(stream, "{}\r\n", response.code).context("Writing Http code")?;

    //writing header
    if response.body.is_some() {
        write!(
            stream,
            "{}{}",
            response.content_type_header(),
            response.content_length_header().unwrap()
        )
        .context("Writing content type header and content length header")?;
    }

    let accept_encoding = request.headers.get("accept-encoding");

    let gzip_encoding = is_gzip_encoding_requested(accept_encoding);

    if gzip_encoding {
        write!(stream, "Content-Encoding: gzip\r\n")
            .context("writing content encoding gzip header")?;
    }

    // writing crlf for headers
    write!(stream, "\r\n").context("writing crlf for headers")?;

    // writing body
    if let Some(body) = &response.body {
        write!(stream, "{}", body).context("writing body")?;
    }
    //flush the stream
    stream
        .flush()
        .context("flushing write so that everything goes out")?;
    Ok(())
}

fn is_gzip_encoding_requested(encodings: Option<&String>) -> bool {
    encodings
        .map(|encoding| {
            let requested_encodings = encoding
                .split(",")
                .map(|encoding| encoding.trim().to_lowercase())
                .collect::<Vec<String>>();

            requested_encodings
                .iter()
                .any(|encoding| encoding == "gzip")
        })
        .unwrap_or_default()
}
