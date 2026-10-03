use http_server::run;
use std::process::exit;

fn main() {
    match run() {
        Ok(_) => println!("server exited"),
        Err(error) => {
            eprintln!("connection error : {}", error);
            exit(1);
        }
    }
}
