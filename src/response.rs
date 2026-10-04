use anyhow::{Context, Ok, Result};
use std::{fmt::Display, io::Write, net::TcpStream};

#[derive(Debug)]
pub enum HttpCode {
    Ok,
    NotFound,
}

impl Display for HttpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (number, message) = match self {
            Self::Ok => (200, "OK"),
            Self::NotFound => (404, "Not Found"),
        };

        write!(f, "{number} {message}")
    }
}

#[derive(Debug)]
pub struct Response {
    pub code: HttpCode,
    pub body: String,
}

impl Response {
    pub fn build(&self) -> Result<String> {
        let code = &self.code;
        let body = &self.body;
        let response = format!("HTTP/1.1 {code}\r\n\r\n{body}");
        Ok(response)
    }
}

pub fn send_response(stream: &mut TcpStream, response: Response) -> Result<()> {
    stream
        .write_all(
            response
                .build()
                .context("building response string")?
                .as_bytes(),
        )
        .context("writing all response data")?;
    stream
        .flush()
        .context("flushing write so that everything goes out")?;
    Ok(())
}
