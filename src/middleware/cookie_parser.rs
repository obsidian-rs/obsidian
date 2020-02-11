use async_trait::async_trait;
use cookie::{Cookie, CookieJar};
use hyper::header;

use crate::app::EndpointExecutor;
use crate::context::Context;
use crate::middleware::Middleware;
use crate::{Body, Response};

#[derive(Default)]
pub struct CookieParserData {
    cookie_jar: CookieJar,
}

impl CookieParserData {
    pub fn new() -> Self {
        CookieParserData {
            cookie_jar: CookieJar::new(),
        }
    }

    pub fn cookie_jar(&self) -> &CookieJar {
        &self.cookie_jar
    }

    pub fn cookie_jar_mut(&mut self) -> &mut CookieJar {
        &mut self.cookie_jar
    }
}

#[derive(Default)]
pub struct CookieParser {}

impl CookieParser {
    pub fn new() -> Self {
        CookieParser {}
    }
}

#[async_trait]
impl Middleware for CookieParser {
    async fn handle<'a>(
        &'a self,
        mut context: Context,
        ep_executor: EndpointExecutor<'a>,
    ) -> Response<Body> {
        if let Some(cookie_header) = context.headers().get(header::COOKIE) {
            if let Ok(cookie_str) = cookie_header.to_str() {
                let mut cookie_data = CookieParserData::new();

                cookie_str
                    .split("; ")
                    .map(|x| x.trim().splitn(2, '=').collect::<Vec<&str>>())
                    .for_each(|x| {
                        if let (2, Some(k), Some(v)) = (x.len(), x.first(), x.last()) {
                            cookie_data
                                .cookie_jar_mut()
                                .add_original(Cookie::new((*k).to_string(), (*v).to_string()));
                        }
                    });

                context.add(cookie_data);
            }
        }

        ep_executor.next(context).await
    }
}
