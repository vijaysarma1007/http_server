pub mod echo;
pub mod home;

use crate::{request::Request, response::Response};
use anyhow::{Context, Ok, Result};

pub fn router(request: Request) -> Result<Response> {
    let response = match request.path.to_lowercase().as_str() {
        "/" => home::home().context("processing home request")?,
        "/echo/*" => todo!(),
        _ => todo!(),
    };

    Ok(response)
}
