use std::collections::HashMap;
use std::num::*;
use std::str::FromStr;

#[derive(Default)]
pub struct Params {
    params_map: HashMap<String, Vec<String>>,
}

impl Params {
    pub fn new(params_map: HashMap<String, Vec<String>>) -> Self {
        Params { params_map }
    }

    pub fn get_param(&self, key: &str) -> Option<&String> {
        if let Some(param) = self.params_map.get(key) {
            param.first()
        } else {
            None
        }
    }

    pub fn get_params(&self, key: &str) -> Option<&Vec<String>> {
        self.params_map.get(key)
    }

    pub fn add_params(&mut self, key: String, val: String) {
        self.params_map.entry(key).or_insert(Vec::new()).push(val)
    }

    pub fn is_empty(&self) -> bool {
        self.params_map.is_empty()
    }
}

pub trait FromParam: Sized {
    type Err;

    fn from_params(src: &Params, key: &str) -> Result<Self, Self::Err>;
}

impl FromParam for Vec<String> {
    type Err = ();
    fn from_params(src: &Params, key: &str) -> Result<Self, Self::Err> {
        match src.get_params(key) {
            Some(params) => Ok(params.clone()),
            _ => Err(()),
        }
    }
}

macro_rules! from_params_impl {
    ($($t:ty)*) => {$(
        impl FromParam for $t {
            type Err = <$t as FromStr>::Err;
            fn from_params(src: &Params, key: &str) -> Result<Self, Self::Err> {
                src.get_param(key).unwrap().parse()
            }
        }
    )*}
}

from_params_impl! {
    isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128
    NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize
    NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize
    f32 f64 bool char String
}
