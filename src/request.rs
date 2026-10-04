use crate::method::Method;
use anyhow::{Context, Ok, Result};
use bytes::Buf;
use bytes::buf::Reader;
use std::io::{BufRead, Read};
use std::net::TcpStream;

const BUFFER_SIZE: usize = 1024;
const SPACE: u8 = b' ';

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
}

pub fn process_request(stream: &mut TcpStream) -> Result<Request> {
    let raw_request = read_stream(stream).context("Reading stream")?;
    let request = parse_raw_request(raw_request).context("Parsing raw request")?;

    Ok(request)
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
