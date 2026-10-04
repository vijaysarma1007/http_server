use anyhow::Result;

use crate::response::Response;

pub fn home() -> Result<Response>{
    Ok(Response { code: crate::response::HttpCode::Ok, body: "".to_owned() })
}
