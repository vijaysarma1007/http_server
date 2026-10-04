mod method;
mod request;
mod response;
mod routes;

use anyhow::{Context, Ok, Result};
use request::process_request;
use response::{HttpCode, send_response};
use std::net::TcpListener;

// this buffer size could cause problem if it happens to be exactly what we end on

pub fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    dbg!("starting to listen");
    for stream in listener.incoming() {
        let mut stream = stream?;
        let request = process_request(&mut stream).context("processing stream into request")?;
        let response = routes::router(request).context("Routing request")?;
        // let response_code = if request.path == "/" {
        //     HttpCode::Ok
        // } else {
        //     HttpCode::NotFound
        // };

        send_response(&mut stream, response).context("sending response")?;
    }

    Ok(())
}
