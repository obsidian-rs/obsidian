use super::RequestData;
use super::ResponseBuilder;

pub trait EndPointHandler:
    Fn(RequestData, ResponseBuilder) -> ResponseBuilder + Send + Sync + 'static
{
}

impl<T> EndPointHandler for T where
    T: Fn(RequestData, ResponseBuilder) -> ResponseBuilder + Send + Sync + 'static
{
}
