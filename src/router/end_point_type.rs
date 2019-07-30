use crate::context::Context;
use super::{ResponseBuilder};

pub trait EndPointHandler:
    Fn(Context, ResponseBuilder) -> ResponseBuilder + Send + Sync + 'static
{
}

impl<T> EndPointHandler for T where
    T: Fn(Context, ResponseBuilder) -> ResponseBuilder + Send + Sync + 'static
{
}
