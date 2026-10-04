use anyhow::{Context, Ok, Result};
use std::{fmt::Display, io::Write, net::TcpStream};

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

pub fn send_response(stream: &mut TcpStream, response_code: HttpCode) -> Result<()> {
    let response = format!("HTTP/1.1 {response_code}\r\n\r\n");
    stream
        .write_all(response.as_bytes())
        .context("writing all response data")?;
    stream
        .flush()
        .context("flushing write so that everything goes out")?;
    Ok(())
}
