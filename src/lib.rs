mod method;
mod request;
mod response;
mod routes;

use anyhow::{Context, Ok, Result};
use request::process_request;
use response::send_response;
use std::{net::TcpListener, thread};

// this buffer size could cause problem if it happens to be exactly what we end on

//Measure-Command { curl.exe -H "blah: blah" http://localhost:4221/user-agent | Out-Host }
pub fn run(port: u32, directory: Option<String>) -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    dbg!("starting to listen");
    let listener = listener.try_clone().context("cloning listener")?;
    for stream in listener.incoming() {
        let path = directory.clone();
        thread::spawn(move || {
            let mut stream = stream.unwrap();
            let request = process_request(&mut stream).unwrap();
            let response = routes::router(&request, path.as_deref()).unwrap();
            send_response(&mut stream, response, &request).unwrap();
        });
    }
    Ok(())
}
