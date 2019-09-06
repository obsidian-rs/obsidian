use serde::de::{self, DeserializeSeed, IntoDeserializer, MapAccess, SeqAccess, Visitor};
use serde::forward_to_deserialize_any;
use serde::Deserialize;

use serde::de::value::Error;
use std::borrow::Cow;
use std::collections::HashMap;

pub fn from_cow_form<'de, T>(s: &'de HashMap<Cow<'de, str>, Cow<'de, [String]>>) -> Result<T, Error>
where
    T: Deserialize<'de>,
{
    let mut deserializer = FormDeserializer::from_cow_form(s.into_iter().peekable());
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

struct FormDeserializer<'de> {
    input: std::iter::Peekable<
        std::collections::hash_map::Iter<
            'de,
            std::borrow::Cow<'de, str>,
            std::borrow::Cow<'de, [String]>,
        >,
    >,
}

impl<'de> FormDeserializer<'de> {
    pub fn from_cow_form(
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
        visitor.visit_map(FromMap::new(self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input.peek().unwrap().0)
    }

    forward_to_deserialize_any! {
        bool
        u8
        u16
        u32
        u64
        i8
        i16
        i32
        i64
        f32
        f64
        char
        str
        option
        bytes
        byte_buf
        unit_struct
        newtype_struct
        tuple_struct
        struct
        string
        tuple
        enum
        ignored_any
        unit
        seq
        map
    }
}

macro_rules! from_string_forms_impl {
    ($($t:ty => $method:ident)*) => {$(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: de::Visitor<'de>
            {
                self.input[0].parse::<$t>().unwrap()
                    .into_deserializer().$method(visitor)
            }
    )*}
}

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
        visitor.visit_char(self.input[0].chars().next().unwrap())
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
        if self.de.input.len() == 1 {
            return Ok(None);
        }

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
        let x = self.de.input.peek().unwrap();
        seed.deserialize(&mut FormValueDeserializer::new(x.1))
    }
}
