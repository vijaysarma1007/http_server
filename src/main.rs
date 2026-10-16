use http_server::run;
use std::{env, process::exit};

fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u32>().ok())
        .unwrap_or(4221);
    match run(port) {
        Ok(_) => println!("server exited"),
        Err(error) => {
            eprintln!("connection error : {}", error);
            exit(1);
        }
    }
}
