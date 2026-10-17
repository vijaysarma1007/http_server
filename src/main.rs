use anyhow::{Context, Ok, Result};
use http_server::run;
use std::{
    env::{self, args},
    process::exit,
};

fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u32>().ok())
        .unwrap_or(4221);

    let directory = get_directory().context("getting directory path").unwrap();

    println!("directory: {:?}", directory);
    match run(port, directory) {
        Result::Ok(_) => println!("server exited"),
        Err(error) => {
            eprintln!("connection error : {}", error);
            exit(1);
        }
    }
}

fn get_directory() -> Result<Option<String>> {
    let mut args = args();
    args.next();

    let directory = if args.next().is_some_and(|arg| arg == "--directory") {
        args.next()
    } else {
        None
    };

    println!("directory:{directory:?}");

    Ok(directory)
}
