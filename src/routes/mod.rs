pub mod echo;
pub mod home;

use crate::{
    request::Request,
    response::{HttpCode, Response},
};
use anyhow::{Context, Ok, Result, bail};

pub fn router(request: Request) -> Result<Response> {
    let mut segments = request.path.trim_matches('/').split('/');
    let response = match segments.next() {
        Some("") => home::home().context("processing home request")?,
        Some("echo") => echo::echo(segments.next()).context("processing path params")?,
        Some(_) => Response {
            code: HttpCode::NotFound,
            body: Some(String::new()),
        },
        None => {
            bail!("Did not get any segments");
        }
    };

    Ok(response)
}
