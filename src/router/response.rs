use super::{ResponseBody, ResponseResult, CustomResponder, Responder};

use hyper::header;
use serde::ser::Serialize;
use serde_json;
use async_std::fs;

use crate::{Response, StatusCode};

pub fn body(body: impl ResponseBody) -> ResponseResult {
    let body = body.into_body();
    Response::builder().status(StatusCode::OK).body(body)
}

pub fn json(body: impl Serialize) -> CustomResponder<String> {
    match serde_json::to_string(&body) {
        Ok(val) => val.status(StatusCode::OK).header(header::CONTENT_TYPE, "application/json"),
        Err(err) => std::error::Error::description(&err).to_string().status(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn file(file_path: &str) -> impl Responder {
    match fs::read_to_string(file_path.to_string()).await {
        Ok(content) => {
            content.status(StatusCode::OK)
        },
        Err(err) => {
            dbg!(&err);
            std::error::Error::description(&err).to_string().status(StatusCode::NOT_FOUND)
        },
    }
}
