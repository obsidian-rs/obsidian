use super::Responder;
use crate::context::Context;

use async_trait::async_trait;
use hyper::{header, Body, Response};
use std::future::Future;

pub type ResponseResult = http::Result<Response<Body>>;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn call(&self, ctx: Context) -> ResponseResult;
}

#[async_trait]
impl<T, F> Handler for T
where
    T: Fn(Context) -> F + Send + Sync + 'static,
    F: Future + Send + 'static,
    F::Output: Responder,
{
    async fn call(&self, ctx: Context) -> ResponseResult {
        let response = (self)(ctx).await.respond_to();

        let mut res = Response::builder();
        if let Some(headers) = response.headers() {
            if let Some(response_headers) = res.headers_mut() {
                headers.iter().for_each(move |(key, value)| {
                    response_headers.insert(key, header::HeaderValue::from_static(value));
                });
            }
        }

        res.status(response.status()).body(response.body())
    }
}
