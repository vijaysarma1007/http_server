pub mod echo;
pub mod handle_file;
pub mod home;
pub mod save_file;
pub mod user_agent;

use crate::{
    method::Method,
    request::Request,
    response::{HttpCode, Response},
};
use anyhow::{Context, Ok, Result, bail};

pub fn router(request: &Request, directory: Option<&str>) -> Result<Response> {
    let mut segments = request.path.trim_matches('/').split('/');
    let response = match segments.next() {
        Some("") => home::home().context("processing home request")?,
        Some("echo") => echo::echo(segments.next()).context("processing path params")?,
        Some("user-agent") => {
            user_agent::user_agent(&request).context("Running user agent hander")?
        }
        Some("files") => match request.method {
            Method::Get => handle_file::handle_file(segments.next(), directory)
                .context("handling file route")?,
            Method::Post => save_file::save_file(directory, segments.next(), &request)
                .context("saving file handler")?,
        },
        Some(_) => Response {
            code: HttpCode::NotFound,
            body: Some(String::new()),
            content_type: crate::response::ContentType::TextPlain,
        },
        None => {
            bail!("Did not get any segments");
        }
    };
    println!("response {:?}", response);
    Ok(response)
}
