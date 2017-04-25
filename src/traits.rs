use std;

pub trait FromHashMap {
    fn from_hashmap(&std::collections::HashMap<&str, &str>) -> Self;
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