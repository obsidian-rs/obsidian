use async_trait::async_trait;
use cookie::Cookie;
use hyper::header;

use crate::app::EndpointExecutor;
use crate::context::{cookies::CookieParserData, Context};
use crate::middleware::Middleware;
use crate::router::ContextResult;

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
    ) -> ContextResult {
        if let Some(cookie_header) = context.headers().get(header::COOKIE) {
            if let Ok(cookie_str) = cookie_header.to_str() {
                let mut cookie_data = CookieParserData::new();

                cookie_str
                    .split("; ")
                    .map(|cookie_str_part| Cookie::parse(cookie_str_part.to_owned()))
                    .for_each(|cookie_result| {
                        if let Ok(cookie) = cookie_result {
                            cookie_data.cookie_jar_mut().add_original(cookie);
                        }
                    });

                context.add(cookie_data);
            }
        }

        ep_executor.next(context).await
    }
}
