use crate::response::Response;
use anyhow::{Ok, Result, bail};

pub fn echo(path_param: Option<&str>) -> Result<Response> {
    let Some(param) = path_param else {
        bail!("missing path params");
    };

    let response = Response {
        code: crate::response::HttpCode::Ok,
        body: Some(param.to_owned()),
    };

    Ok(response)
}
