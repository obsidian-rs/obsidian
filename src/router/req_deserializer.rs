use serde::de::{self, DeserializeSeed, IntoDeserializer, MapAccess, SeqAccess, Visitor};
use serde::forward_to_deserialize_any;
use serde::ser;
use serde::Deserialize;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

/// Parse merged forms key, value pair get from form_urlencoded into a user defined struct
/// Key and Value should be in Cow pointer
///
/// # Example
///
/// ```
/// # use obsidian::router::from_cow_map;
/// # use hyper::{Body, Request, body, body::Buf};
/// # use url::form_urlencoded;
/// # use serde::*;
/// # use std::collections::HashMap;
/// # use std::borrow::Cow;
/// # use async_std::task;
///
/// #[derive(Deserialize, Debug, PartialEq)]
/// struct Example {
///     field1: Vec<i32>,
///     field2: i32,
/// }
/// task::block_on(
/// async {
///     let body = Request::new(Body::from("field1=1&field1=2&field2=12")).into_body();
///  
///     let buf = match body::aggregate(body).await {
///         Ok(buf) => buf,
///         Err(e) => {
///             println!("{}", e);
///             panic!()
///         }
///     };
///         
///     let mut parsed_form_map: HashMap<String, Vec<String>> = HashMap::default();
///     let mut cow_form_map = HashMap::<Cow<str>, Cow<[String]>>::default();
///         
///     // Parse and merge chunks with same name key
///     form_urlencoded::parse(buf.bytes())
///         .into_owned()
///         .for_each(|(key, val)| {
///             parsed_form_map.entry(key).or_insert(vec![]).push(val);
///         });
///         
///     // Wrap vec with cow pointer
///     parsed_form_map.iter().for_each(|(key, val)| {
///         cow_form_map
///             .entry(std::borrow::Cow::from(key))
///             .or_insert(std::borrow::Cow::from(val));
///     });
///
///     let actual_result: Example = from_cow_map(&cow_form_map).unwrap();
///     let expected_result = Example{field1: vec![1,2], field2:12};
///        
///     assert_eq!(actual_result, expected_result);
/// })
/// ```
pub fn from_cow_map<'de, T, S: ::std::hash::BuildHasher>(
    s: &'de HashMap<Cow<'de, str>, Cow<'de, [String]>, S>,
) -> Result<T, Error>
where
    T: Deserialize<'de>,
{
    let mut deserializer = FormDeserializer::from_cow_map(s.iter().peekable());
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

/// Deserializer for merged hashmap forms.
struct FormDeserializer<'de> {
    input: std::iter::Peekable<
        std::collections::hash_map::Iter<
            'de,
            std::borrow::Cow<'de, str>,
            std::borrow::Cow<'de, [String]>,
        >,
    >,
}

macro_rules! from_string_forms_key_impl {
    ($($t:ty => $method:ident)*) => {$(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: de::Visitor<'de>
            {
                match self.input.peek() {
                    Some(key) => key.0.clone().into_deserializer().$method(visitor),
                    _ => Err(Error::NoneError),
                }
            }
    )*}
}

impl<'de> FormDeserializer<'de> {
    pub fn from_cow_map(
        input: std::iter::Peekable<
            std::collections::hash_map::Iter<
                'de,
                std::borrow::Cow<'de, str>,
                std::borrow::Cow<'de, [String]>,
            >,
        >,
    ) -> Self {
        FormDeserializer { input }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut FormDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(FromMap::new(self))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.input.peek() {
            Some(key) => visitor.visit_str(key.0),
            _ => Err(Error::NoneError),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    forward_to_deserialize_any! {
        char
        option
        bytes
        byte_buf
        unit_struct
        newtype_struct
        tuple_struct
        struct
        tuple
        enum
        ignored_any
        unit
        seq
    }

    from_string_forms_key_impl! {
        bool => deserialize_bool
        u8 => deserialize_u8
        u16 => deserialize_u16
        u32 => deserialize_u32
        u64 => deserialize_u64
        i8 => deserialize_i8
        i16 => deserialize_i16
        i32 => deserialize_i32
        i64 => deserialize_i64
        f32 => deserialize_f32
        f64 => deserialize_f64
    }
}

macro_rules! from_string_forms_impl {
    ($($t:ty => $method:ident)*) => {$(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: de::Visitor<'de>
            {
                match self.input[0].parse::<$t>() {
                    Ok(result) => result.into_deserializer().$method(visitor),
                    Err(e) => Err(Error::Message(format!("{}", e))),
                }
            }
    )*}
}

// Deserializer for value of merged hashmap forms.
struct FormValueDeserializer<'de> {
    input: &'de [std::string::String],
}

impl<'de> FormValueDeserializer<'de> {
    pub fn new(input: &'de [std::string::String]) -> Self {
        FormValueDeserializer { input }
    }
}

impl<'de> de::Deserializer<'de> for &mut FormValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(FromSeq::new(self))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.input[0].clone())
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(&self.input[0])
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.input[0].chars().next() {
            Some(val) => visitor.visit_char(val),
            _ => Err(Error::NoneError),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if self.input.starts_with(&[String::default()]) {
            self.input = &self.input[1..];
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if self.input.starts_with(&[String::default()]) {
            self.input = &self.input[1..];
            visitor.visit_unit()
        } else {
            visitor.visit_unit()
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    forward_to_deserialize_any! {
        bytes
        byte_buf
        ignored_any
        map
        struct
        tuple_struct
        enum
    }

    from_string_forms_impl! {
        bool => deserialize_bool
        u8 => deserialize_u8
        u16 => deserialize_u16
        u32 => deserialize_u32
        u64 => deserialize_u64
        i8 => deserialize_i8
        i16 => deserialize_i16
        i32 => deserialize_i32
        i64 => deserialize_i64
        f32 => deserialize_f32
        f64 => deserialize_f64
    }
}

struct FromSeq<'a, 'de: 'a> {
    de: &'a mut FormValueDeserializer<'de>,
    first: bool,
}

impl<'a, 'de> FromSeq<'a, 'de> {
    fn new(de: &'a mut FormValueDeserializer<'de>) -> Self {
        FromSeq { de, first: true }
    }
}

impl<'de, 'a> SeqAccess<'de> for FromSeq<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.input.len() == 1 && !self.first {
            return Ok(None);
        }

        // Only start moving slices after processing
        if !self.first {
            self.de.input = &self.de.input[1..];
        }
        self.first = false;

        seed.deserialize(&mut *self.de).map(Some)
    }
}

struct FromMap<'a, 'de: 'a> {
    de: &'a mut FormDeserializer<'de>,
    first: bool,
}

impl<'a, 'de> FromMap<'a, 'de> {
    fn new(de: &'a mut FormDeserializer<'de>) -> Self {
        FromMap { de, first: true }
    }
}

impl<'de, 'a> MapAccess<'de> for FromMap<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: DeserializeSeed<'de>,
    {
        if !self.first {
            self.de.input.next();
        }

        self.first = false;

        match self.de.input.peek() {
            Some(_x) => seed.deserialize(&mut *self.de).map(Some),
            _ => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.de.input.peek() {
            Some(val) => seed.deserialize(&mut FormValueDeserializer::new(val.1)),
            _ => Err(Error::NoneError),
        }
    }
}

/// Error for request deserializer
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
    NoneError,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Message(ref msg) => msg,
            Error::NoneError => "Input should not be None",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::task;
    use hyper::{body, body::Buf, Body, Request};
    use url::form_urlencoded;

    #[derive(Deserialize, Debug, PartialEq)]
    struct VecAndSingleVariableStruct {
        field1: Vec<String>,
        field2: i32,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct VecStruct {
        field1: Vec<i32>,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct VecWithDefaultStruct {
        field1: Vec<i32>,
        #[serde(default)]
        field2: i32,
    }

    #[test]
    fn test_deserialize_to_struct_with_vec_and_single_variable() {
        task::block_on(async {
            let body = Request::new(Body::from("field1=abc&field1=xyz&field2=12")).into_body();
            let buf = match body::aggregate(body).await {
                Ok(buf) => buf,
                Err(_e) => {
                    panic!("Body parsing fail");
                }
            };
            let mut parsed_form_map: HashMap<String, Vec<String>> = HashMap::default();
            let mut cow_form_map = HashMap::<Cow<str>, Cow<[String]>>::default();
            // Parse and merge chunks with same name key
            form_urlencoded::parse(buf.bytes())
                .into_owned()
                .for_each(|(key, val)| {
                    parsed_form_map.entry(key).or_insert(vec![]).push(val);
                });
            // Wrap vec with cow pointer
            parsed_form_map.iter().for_each(|(key, val)| {
                cow_form_map
                    .entry(std::borrow::Cow::from(key))
                    .or_insert(std::borrow::Cow::from(val));
            });

            let actual_result: VecAndSingleVariableStruct = from_cow_map(&cow_form_map).unwrap();
            let expected_result = VecAndSingleVariableStruct {
                field1: vec!["abc".to_string(), "xyz".to_string()],
                field2: 12,
            };
            assert_eq!(actual_result, expected_result);
        })
    }

    #[test]
    fn test_deserialize_to_struct_with_vec() {
        task::block_on(async {
            let body = Request::new(Body::from("field1=1&field1=2")).into_body();
            let buf = match body::aggregate(body).await {
                Ok(buf) => buf,
                Err(_e) => {
                    panic!("Body parsing fail");
                }
            };
            let mut parsed_form_map: HashMap<String, Vec<String>> = HashMap::default();
            let mut cow_form_map = HashMap::<Cow<str>, Cow<[String]>>::default();
            // Parse and merge chunks with same name key
            form_urlencoded::parse(buf.bytes())
                .into_owned()
                .for_each(|(key, val)| {
                    parsed_form_map.entry(key).or_insert(vec![]).push(val);
                });
            // Wrap vec with cow pointer
            parsed_form_map.iter().for_each(|(key, val)| {
                cow_form_map
                    .entry(std::borrow::Cow::from(key))
                    .or_insert(std::borrow::Cow::from(val));
            });

            let actual_result: VecStruct = from_cow_map(&cow_form_map).unwrap();
            let expected_result = VecStruct { field1: vec![1, 2] };
            assert_eq!(actual_result, expected_result);
        })
    }

    #[test]
    fn test_deserialize_to_struct_with_extra_form_value() {
        task::block_on(async {
            let body = Request::new(Body::from("field1=1&field1=2&field2=12")).into_body();
            let buf = match body::aggregate(body).await {
                Ok(buf) => buf,
                Err(_e) => {
                    panic!("Body parsing fail");
                }
            };
            let mut parsed_form_map: HashMap<String, Vec<String>> = HashMap::default();
            let mut cow_form_map = HashMap::<Cow<str>, Cow<[String]>>::default();
            // Parse and merge chunks with same name key
            form_urlencoded::parse(buf.bytes())
                .into_owned()
                .for_each(|(key, val)| {
                    parsed_form_map.entry(key).or_insert(vec![]).push(val);
                });
            // Wrap vec with cow pointer
            parsed_form_map.iter().for_each(|(key, val)| {
                cow_form_map
                    .entry(std::borrow::Cow::from(key))
                    .or_insert(std::borrow::Cow::from(val));
            });

            let actual_result: VecStruct = from_cow_map(&cow_form_map).unwrap();
            let expected_result = VecStruct { field1: vec![1, 2] };
            assert_eq!(actual_result, expected_result);
        })
    }

    #[test]
    fn test_deserialize_to_struct_with_extra_struct_field() {
        task::block_on(async {
            let body = Request::new(Body::from("field1=1&field1=2")).into_body();
            let buf = match body::aggregate(body).await {
                Ok(buf) => buf,
                Err(_e) => {
                    panic!("Body parsing fail");
                }
            };
            let mut parsed_form_map: HashMap<String, Vec<String>> = HashMap::default();
            let mut cow_form_map = HashMap::<Cow<str>, Cow<[String]>>::default();
            // Parse and merge chunks with same name key
            form_urlencoded::parse(buf.bytes())
                .into_owned()
                .for_each(|(key, val)| {
                    parsed_form_map.entry(key).or_insert(vec![]).push(val);
                });
            // Wrap vec with cow pointer
            parsed_form_map.iter().for_each(|(key, val)| {
                cow_form_map
                    .entry(std::borrow::Cow::from(key))
                    .or_insert(std::borrow::Cow::from(val));
            });

            let actual_result: VecWithDefaultStruct = from_cow_map(&cow_form_map).unwrap();
            let expected_result = VecWithDefaultStruct {
                field1: vec![1, 2],
                field2: i32::default(),
            };
            assert_eq!(actual_result, expected_result);
        })
    }

    #[test]
    fn test_deserialize_to_map_type() {
        task::block_on(async {
            let body = Request::new(Body::from("field1=1&field1=2&field2=3")).into_body();
            let buf = match body::aggregate(body).await {
                Ok(buf) => buf,
                Err(_e) => {
                    panic!("Body parsing fail");
                }
            };
            let mut parsed_form_map: HashMap<String, Vec<String>> = HashMap::default();
            let mut cow_form_map = HashMap::<Cow<str>, Cow<[String]>>::default();
            // Parse and merge chunks with same name key
            form_urlencoded::parse(buf.bytes())
                .into_owned()
                .for_each(|(key, val)| {
                    parsed_form_map.entry(key).or_insert(vec![]).push(val);
                });
            // Wrap vec with cow pointer
            parsed_form_map.iter().for_each(|(key, val)| {
                cow_form_map
                    .entry(std::borrow::Cow::from(key))
                    .or_insert(std::borrow::Cow::from(val));
            });

            let actual_result: HashMap<String, Vec<i32>> = from_cow_map(&cow_form_map).unwrap();
            let mut expected_result: HashMap<String, Vec<i32>> = HashMap::default();

            expected_result
                .entry("field1".to_string())
                .or_insert(vec![])
                .push(1);
            expected_result
                .entry("field1".to_string())
                .or_insert(vec![])
                .push(2);
            expected_result
                .entry("field2".to_string())
                .or_insert(vec![])
                .push(3);
            assert_eq!(actual_result, expected_result);
        })
    }
}
