use anyhow::{Context, Error, bail};

#[derive(Debug)]
pub enum Method {
    Get,
}

impl TryFrom<Vec<u8>> for Method {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> std::prelude::v1::Result<Self, Self::Error> {
        let method_string =
            String::from_utf8(value).context("Converting bytes to method string")?;

        Ok(match method_string.to_uppercase().trim() {
            "GET" => Self::Get,
            _ => bail!("Unknown Method"),
        })
    }
}
