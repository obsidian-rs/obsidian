use super::{ResponseBody, ResponseResult};

use hyper::header;
use serde::ser::Serialize;
use serde_json;
use async_std::fs;

use crate::{Response, StatusCode};

static NOTFOUND: &[u8] = b"Not Found";

pub fn body(body: impl ResponseBody) -> ResponseResult {
    let body = body.into_body();
    Response::builder().status(StatusCode::OK).body(body)
}

pub fn json(body: impl Serialize, status_code: StatusCode) -> ResponseResult {
    let serialized_obj = match serde_json::to_string(&body) {
        Ok(val) => val,
        Err(e) => e.to_string(),
    };

    let body = serialized_obj.into_body();

    Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .status(status_code)
        .body(body)
}

pub async fn file(file_path: &str) -> ResponseResult {
    match fs::read(file_path.to_string()).await {
        Ok(buf) => {
            Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(buf.into())
                        .unwrap())
        },
        Err(err) => {
            dbg!(err);
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        },
    }
}
