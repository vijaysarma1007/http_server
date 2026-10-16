use anyhow::Result;

use crate::{request::Request, response::Response};

pub fn user_agent(request: &Request) -> Result<Response> {
    let user_agent = request.headers.get("user-agent").map(ToOwned::to_owned);

    Ok(Response {
        code: crate::response::HttpCode::Ok,
        body: user_agent
    })
}
