use async_trait::async_trait;
use cookie::Cookie;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::app::EndpointExecutor;
use crate::context::{session::SessionStorage, Context};
use crate::middleware::Middleware;
use crate::router::ContextResult;

pub struct CookieSession<T>
where
    T: SessionStorage,
{
    storage: T,
}

impl<T> CookieSession<T>
where
    T: SessionStorage,
{
    pub fn new(storage: T) -> Self {
        CookieSession { storage }
    }
}

#[async_trait]
impl<T> Middleware for CookieSession<T>
where
    T: SessionStorage + 'static,
{
    async fn handle<'a>(
        &'a self,
        mut context: Context,
        ep_executor: EndpointExecutor<'a>,
    ) -> ContextResult {
        let mut session_id = None;

        if let Some(id) = context.cookie("obsd_session_id") {
            if let Some(session) = self.storage.get_session(id.value()) {
                if session.is_alive() {
                    session_id = Some(id.value().to_owned());
                    context.add(session);
                }
            }
        }

        let ctx_result = ep_executor.next(context).await;

        match ctx_result {
            Ok(mut ctx) => {
                if let Some(ctx_session) = ctx.session() {
                    let session = ctx_session.clone();

                    match session_id {
                        Some(id) => {
                            self.storage.set_session(&id, session);
                        }
                        _ => {
                            let mut id: String =
                                thread_rng().sample_iter(&Alphanumeric).take(30).collect();

                            while self.storage.get_session(&id).is_some() {
                                if self.storage.get_session(&id).unwrap().is_alive() {
                                    id = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
                                } else {
                                    // replace expired session
                                    break;
                                }
                            }

                            self.storage.set_session(&id, session);
                            if let Some(res) = ctx.take_response() {
                                let cookie = Cookie::new("obsd_session_id", id);
                                *ctx.response_mut() = Some(res.set_cookie(cookie));
                            }
                        }
                    }
                }

                Ok(ctx)
            }
            Err(err) => Err(err),
        }
    }
}
