use std;

pub trait StructMap {
    fn from_hashmap(&std::collections::HashMap<&str, &str>) -> Self;
    fn from_tuples(pairs: Vec<(&str, &str)>) -> Self where Self: std::marker::Sized {
        let map: std::collections::HashMap<&str, &str> = pairs.into_iter().collect();
        Self::from_hashmap(&map)
    }
}

pub trait FromStr : std::str::FromStr {
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

impl FromStr for String {
    fn from_str(s: &str) -> Result<String, std::string::ParseError> {
        Ok(s.to_owned())
    }
}

impl FromStr for u32 {
    fn from_str(s: &str) -> Result<u32, std::num::ParseIntError> {
        std::str::FromStr::from_str(s)
    }
}

impl FromStr for f32 {
    fn from_str(s: &str) -> Result<f32, std::num::ParseFloatError> {
        std::str::FromStr::from_str(s)
    }
}

impl FromStr for bool {
    fn from_str(s: &str) -> Result<bool, std::str::ParseBoolError> {
        match s {
            "1" => Ok(true),
            "0" => Ok(false),
            _   => std::str::FromStr::from_str(""),
        }
    }
}