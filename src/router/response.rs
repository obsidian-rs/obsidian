use super::{ResponseBody, ResponseResult};

use futures::future::Future;
use hyper::header;
use serde::ser::Serialize;
use serde_json;
use tokio_fs;
use tokio_io;

use crate::{Body, Response, StatusCode};

static NOTFOUND: &[u8] = b"Not Found";

pub fn body(body: impl ResponseBody) -> ResponseResult {
    let body = body.into_body();
    Response::builder().status(StatusCode::OK).body(body)
}

pub fn json(body: impl Serialize, status_code: StatusCode) -> ResponseResult {
    let serialized_obj = match serde_json::to_string(&body) {
        Ok(val) => val,
        Err(e) => std::error::Error::description(&e).to_string(),
    };

    let body = serialized_obj.into_body();

    Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .status(status_code)
        .body(body)
}

pub fn file(file_path: &str) -> ResponseResult {
    let response = tokio_fs::file::File::open(file_path.to_string())
        .and_then(|file| {
            let buf: Vec<u8> = Vec::new();
            tokio_io::io::read_to_end(file, buf)
                .and_then(|item| {
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(item.1.into())
                        .unwrap())
                })
                .or_else(|_| {
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap())
                })
        })
        .or_else(|err| {
            dbg!(&err);
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        })
        .wait();

    response
}
