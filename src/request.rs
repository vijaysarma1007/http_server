use crate::method::Method;
use anyhow::{Context, Ok, Result, bail};
use bytes::Buf;
use bytes::buf::Reader;
use std::collections::HashMap;
use std::io::{BufRead, Read};
use std::net::TcpStream;

const BUFFER_SIZE: usize = 1024;
const SPACE: u8 = b' ';

pub type Headers = HashMap<String, String>;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: String,
    pub headers: Headers,
    pub body: Option<String>,
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
    let protocol =
        parse_protocol_form_request(&mut reader).context("parsing protocol from request")?;
    //skip_crlf(&mut reader).context("skipping \r\n")?;
    let headers =
        parse_headers_from_request(&mut reader).context("Parsing headers from request")?;
    println!("about to parse body");
    let body = if matches!(method, Method::Post) {
        Some(parse_body_from_request(&mut reader, &headers).context("parsing body from request")?)
    } else {
        None
    };
    println!("body: {body:?}");

    Ok(Request {
        method,
        path,
        protocol,
        headers,
        body,
    })
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

fn parse_protocol_form_request(request: &mut Reader<&[u8]>) -> Result<String> {
    let mut protocol = String::new();
    request
        .read_line(&mut protocol)
        .context("reading protocol bytes")?;
    Ok(protocol.trim().to_owned())
}

fn parse_headers_from_request(request: &mut Reader<&[u8]>) -> Result<Headers> {
    let mut headers = HashMap::new();
    loop {
        let mut raw_header = String::new();
        request
            .read_line(&mut raw_header)
            .context("reading header bytes")?;
        let header = raw_header.trim();

        if header.is_empty() {
            break;
        }

        let mut header_parts = raw_header.splitn(2, ':');
        let header_name = header_parts
            .next()
            .map(|header| header.trim().to_lowercase());
        let header_value = header_parts.next().map(|header| header.trim().to_owned());

        if header_name.is_none() || header_value.is_none() {
            continue;
        }

        headers.insert(header_name.unwrap(), header_value.unwrap());
    }
    println!("headers: {:?}", headers);
    Ok(headers)
}

fn parse_body_from_request(request: &mut Reader<&[u8]>, headers: &Headers) -> Result<String> {
    let Some(content_length) = headers.get("content-length") else {
        bail!("missing content length header");
    };

    let length = content_length
        .parse::<usize>()
        .context("parsing content length header into number")?;
    let mut buffer = vec![0_u8; length];

    request.read(&mut buffer).context("reading into buffer")?;

    String::from_utf8(buffer).context("converting body bufer to string")
}
