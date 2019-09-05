use super::Responder;
use super::ResponseBuilder;
use crate::context::Context;

pub trait EndPointHandler:
    Fn(Context, ResponseBuilder) -> Box<dyn Responder> + Send + Sync + 'static
{
}

impl<T> EndPointHandler for T where
    T: Fn(Context, ResponseBuilder) -> Box<dyn Responder> + Send + Sync + 'static
{
}

// pub trait EndPointHandler:
//     Fn(Context, ResponseBuilder) -> ResponseBuilder + Send + Sync + 'static
// {
// }

// impl<T> EndPointHandler for T where
//     T: Fn(Context, ResponseBuilder) -> ResponseBuilder + Send + Sync + 'static
// {
// }
