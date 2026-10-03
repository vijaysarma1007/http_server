use anyhow::{Context, Result};
use std::{io::Write, net::TcpListener};

pub fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream?;
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
