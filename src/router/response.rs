use http::{
    header::HeaderName,
    response::{Builder, Response},
    version::Version,
    StatusCode,
};
use hyper::Body;
use std::any::Any;

pub trait ResponseBody {
    fn into_body(self) -> Body;
}

impl ResponseBody for () {
    fn into_body(self) -> Body {
        Body::empty()
    }
}

impl ResponseBody for &'static str {
    fn into_body(self) -> Body {
        Body::from(self)
    }
}

impl ResponseBody for String {
    fn into_body(self) -> Body {
        Body::from(self)
    }
}

impl ResponseBody for Vec<u8> {
    fn into_body(self) -> Body {
        Body::from(self)
    }
}

pub struct ObsidianResponse {
    response_builder: Builder,
    body: Body,
}

impl ObsidianResponse {
    pub fn new() -> Self {
        ObsidianResponse {
            response_builder: Response::builder(),
            body: Body::empty(),
        }
    }

    pub fn header(mut self, key: HeaderName, value: &str) -> Self {
        self.response_builder.header(key, value);
        self
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.response_builder.status(status);
        self
    }

    pub fn version(mut self, version: Version) -> Self {
        self.response_builder.version(version);
        self
    }

    pub fn extension<T>(mut self, extension: T) -> Self
    where
        T: Any + Send + Sync + 'static,
    {
        self.response_builder.extension(extension);
        self
    }

    pub fn body(mut self, body: impl ResponseBody) -> Self {
        self.body = body.into_body();
        self
    }
}

impl Into<Response<Body>> for ObsidianResponse {
    fn into(mut self) -> Response<Body> {
        self.response_builder.body(self.body).unwrap()
    }
}

impl Default for ObsidianResponse {
    fn default() -> Self {
        ObsidianResponse {
            response_builder: Response::builder(),
            body: Body::empty(),
        }
    }
}
