use anyhow::{Context, Error, Ok, Result, bail};
use bytes::Buf;
use bytes::buf::Reader;
use std::{
     fmt::Display, io::{BufRead, Read, Write}, net::{TcpListener, TcpStream},
};

// this buffer size could cause problem if it happens to be exactly what we end on
const BUFFER_SIZE: usize = 1024;
const SPACE: u8 = b' ';

#[derive(Debug)]
struct Request {
    method: Method,
    path: String,
}

#[derive(Debug)]
enum Method {
    Get,
}

enum HttpCode {
    Ok,
    NotFound
}


impl Display for HttpCode {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         let (number, message) = match self {
            Self::Ok => (200, "OK"),
            Self::NotFound => (404, "Not Found")
         };

         write!(f, "{number} {message}")
     }
}

impl TryFrom<Vec<u8>> for Method {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> std::prelude::v1::Result<Self, Self::Error> {
        let method_string =
            String::from_utf8(value).context("Converting bytes to method string")?;

        Ok(match method_string.to_uppercase().trim() {
            "GET" => Self::Get,
            _ => bail!("Unknown Method"),
        })
    }
}

pub fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    dbg!("starting to listen");
    for stream in listener.incoming() {
        let mut stream = stream?;

        let raw_request = read_stream(&mut stream).context("Reading stream")?;
        let request = parse_raw_request(raw_request).context("Parsing raw request")?;

        let response_code = if request.path == "/" { HttpCode::Ok } else { HttpCode::NotFound };

        let response = format!("HTTP/1.1 {response_code}\r\n\r\n");
        stream
            .write_all(response.as_bytes())
            .context("writing all response data")?;
        stream
            .flush()
            .context("flushing write so that everything goes out")?;
    }

    Ok(())
}

fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>> {
    // possible ideas
    // BUfReader and read line
    let mut request: Vec<u8> = vec![];
    loop {
        let mut chunk = [0_u8; BUFFER_SIZE]; // reads the number of chunks
        let how_many_read = stream.read(&mut chunk).context("Reading request chunk")?;

        request.extend_from_slice(&chunk[..how_many_read]);
        dbg!(how_many_read);
        //not sure if correct
        if how_many_read < BUFFER_SIZE {
            break;
        }
    }

    Ok(request)
}

// parse the raw http request that we got from the client
fn parse_raw_request(request: Vec<u8>) -> Result<Request> {
    let mut reader = request.as_slice().reader();
    let method = parse_method_from_request(&mut reader).context("Parsing method")?;
    let path = parse_path_from_request(&mut reader).context("parsing path from request")?;
    Ok(Request { method, path })
}

fn parse_method_from_request(request: &mut Reader<&[u8]>) -> Result<Method> {
    let mut method = vec![];
    request
        .read_until(SPACE, &mut method)
        .context("Getting method bytes")?;
    Method::try_from(method)
}

fn parse_path_from_request(request: &mut Reader<&[u8]>) -> Result<String> {
    let mut path_bytes = vec![];
    request
        .read_until(SPACE, &mut path_bytes)
        .context("Parseing path from request")?;
    Ok(String::from_utf8(path_bytes)
        .context("converting path bytes to string")?
        .trim()
        .to_owned())
}
