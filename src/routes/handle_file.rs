use std::{
    fs::File,
    io::{ErrorKind, Read},
    path::Path,
};

use crate::response::Response;
use anyhow::{Context, Ok, Result};

pub fn handle_file(file_name: Option<&str>, directory: Option<&str>) -> Result<Response> {
    let Some(directory) = directory else {
        return Ok(Response {
            code: crate::response::HttpCode::YourFault,
            body: None,
            content_type: crate::response::ContentType::TextPlain,
        });
    };

    let Some(file_name) = file_name else {
        return Ok(Response {
            code: crate::response::HttpCode::NotFound,
            body: None,
            content_type: crate::response::ContentType::TextPlain,
        });
    };

    let path = Path::new(directory).join(file_name);

    let mut file = match File::open(path) {
        Result::Ok(file) => file,
        Err(error) => {
            let kind = error.kind();
            if let ErrorKind::NotFound = kind {
                return Ok(Response {
                    code: crate::response::HttpCode::NotFound,
                    body: None,
                    content_type: crate::response::ContentType::TextPlain,
                });
            } else {
                return Err(error.into());
            }
        }
    };

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .context("Reading file to string")?;

    Ok(Response {
        code: crate::response::HttpCode::Ok,
        body: Some(contents),
        content_type: crate::response::ContentType::ApplicationOctetStream,
    })
}
