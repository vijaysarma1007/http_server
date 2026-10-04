use anyhow::{Context, Error, Ok, Result, anyhow, bail};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

// this buffer size could cause problem if it happens to be exactly what we end on
const BUFFER_SIZE: usize = 1024;
const SPACE: u8 = b' ';

#[derive(Debug)]
struct Request {
    method: Method,
}

#[derive(Debug)]
enum Method {
    Get,
}

impl TryFrom<Vec<u8>> for Method {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> std::prelude::v1::Result<Self, Self::Error> {
        let method_string =
            String::from_utf8(value).context("Converting bytes to method string")?;

        Ok(match method_string.to_uppercase().as_str() {
            "GET" => Self::Get,
            _ => bail!("Unknown Method"),
        })
    }
}

#[derive(Debug, Default)]
enum RequestParseState {
    #[default]
    Method,
}

pub fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    dbg!("starting to listen");
    for stream in listener.incoming() {
        let mut stream = stream?;

        let raw_request = read_stream(&mut stream).context("Reading stream")?;
        let request = parse_raw_request(raw_request).context("Parsing raw request")?;
        dbg!(request);

        let response = "HTTP/1.1 200 OK\r\n\r\n";
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
    let state = RequestParseState::default();
    let method;
    match state {
        RequestParseState::Method => {
            method =
                parse_method_from_request(&request).context("getting http method from request")?
        }
    };

    Ok(Request { method })
}

fn parse_method_from_request(request: &[u8]) -> Result<Method> {
    let mut method = vec![];
    for &next_byte in request {
        if next_byte != SPACE {
            method.push(next_byte);
        } else {
            break;
        }
    }

    Method::try_from(method)
}
