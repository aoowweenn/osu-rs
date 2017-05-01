use std;

pub trait StructMap {
    fn from_hashmap(&std::collections::HashMap<&str, &str>) -> Self;
    fn from_vec(v: Vec<&str>) -> Self;
    fn from_tuples(pairs: Vec<(&str, &str)>) -> Self
        where Self: std::marker::Sized
    {
        let map: std::collections::HashMap<&str, &str> = pairs.into_iter().collect();
        Self::from_hashmap(&map)
    }
}

pub trait FromStr: Sized {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

impl FromStr for String {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<String, std::string::ParseError> {
        std::str::FromStr::from_str(s)
    }
}

impl FromStr for u32 {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<u32, std::num::ParseIntError> {
        std::str::FromStr::from_str(s)
    }
}

impl FromStr for f32 {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<f32, std::num::ParseFloatError> {
        std::str::FromStr::from_str(s)
    }
}

impl FromStr for bool {
    type Err = std::str::ParseBoolError;

    fn from_str(s: &str) -> Result<bool, std::str::ParseBoolError> {
        match s {
            "1" => Ok(true),
            "0" => Ok(false),
            _ => std::str::FromStr::from_str(""),
        }
    }
}

impl FromStr for ::Mode {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<::Mode, std::string::ParseError> {
        let mode_ind: i32 = std::str::FromStr::from_str(s).unwrap();
        let res = match mode_ind {
            0 => ::Mode::Osu,
            1 => ::Mode::Taiko,
            2 => ::Mode::CatchTheBeat,
            3 => ::Mode::OsuMania,
            _ => ::Mode::Wrong,
        };
        Ok(res)
    }
}

impl Default for ::Mode {
    fn default() -> ::Mode {
        ::Mode::Wrong
    }
}

impl FromStr for Vec<u32> {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Vec<u32>, std::string::ParseError> {
        let res = s.split(',')
            .filter_map(|x| std::str::FromStr::from_str(x).ok())
            .collect::<Vec<_>>();
        Ok(res)
    }
}

impl FromStr for Vec<String> {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Vec<String>, std::string::ParseError> {
        let res = s.split_whitespace()
            .filter_map(|x| Some(x.to_owned()))
            .collect::<Vec<_>>();
        Ok(res)
    }
}