use serde::Serialize;

pub enum ResponseType<T>
where
    T: Serialize,
{
    JSON(T),
}
