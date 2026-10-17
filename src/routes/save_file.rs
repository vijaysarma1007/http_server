use std::{fs::File, io::Write, path::Path};

use anyhow::{Context, Result};

use crate::{request::Request, response::Response};

pub fn save_file(
    directory: Option<&str>,
    file_name: Option<&str>,
    request: &Request,
) -> Result<Response> {
    let Some(directory) = directory else {
        return Ok(Response {
            code: crate::response::HttpCode::YourFault,
            body: None,
            content_type: crate::response::ContentType::TextPlain,
        });
    };

    let Some(file_name) = file_name else {
        return Ok(Response {
            code: crate::response::HttpCode::YourFault,
            body: None,
            content_type: crate::response::ContentType::TextPlain,
        });
    };

    let path = Path::new(directory);
    let mut file = File::create(path.join(file_name)).context("opening file for write")?;
    let Some(body) = &request.body else {
        return Ok(Response {
            code: crate::response::HttpCode::YourFault,
            body: None,
            content_type: crate::response::ContentType::TextPlain,
        });
    };
    let mut body_buffer = body.as_bytes();
    file.write_all(&mut body_buffer)
        .context("writing content into file")?;
    file.flush().context("flushing body")?;

    Ok(Response {
        code: crate::response::HttpCode::Created,
        body: None,
        content_type: crate::response::ContentType::TextPlain,
    })
}
