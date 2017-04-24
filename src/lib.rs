extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate from_hashmap_derive;

use std::collections::HashMap;
use nom::{digit, line_ending, multispace};
use nom::IResult::Done;

trait FromHashMap {
    fn from_hashmap(&HashMap<&str, &str>) -> General;
}


#[derive(Debug, Serialize)]
pub struct Osu {
    version: u32,
    general: General,
}

#[derive(Debug, PartialEq, Default, Serialize, FromHashMap)]
struct General {
    audio_filename: String,
    audio_lead_in:  u32,
    preview_time:   u32,
    countdown:      bool,
    sample_set:     String,
    stack_leniency: f32,
    mode:           u32,
    letterbox_in_breaks: bool,
    widescreen_storyboard: bool,
}

trait FromStr : std::str::FromStr {
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

impl General {
    pub fn from_tuples(pairs: Vec<(&str, &str)>) -> General {
        let map: HashMap<&str, &str> = pairs.into_iter().collect();
        General::from_hashmap(&map)
    }
}

#[derive(Debug, PartialEq, Default, Serialize)]
struct Editor {
    bookmarks: Vec<u32>,
    distance_spacing: f32,
    beat_divisor: u32,
    grid_size: u32,
    timeline_zoom: f32,
}

named!(key_value_pair<&str, (&str, &str)>,
    do_parse!(
           key: take_until_and_consume_s!(":")
        >> opt!(multispace)
        >> val: take_until_and_consume_s!("\r\n")
        >> (key, val)
    )
);

named!(section_general<&str, General>,
        do_parse!(
                                tag_s!("[General]")
           >>                   opt!(multispace)
           >> pairs_with_end:   many_till!(key_value_pair, line_ending)
           >> (General::from_tuples(pairs_with_end.0))
          )
        );

named!(section_editor<&str, Editor>,
        do_parse!(
                                tag_s!("[Editor]")
           >>                   opt!(multispace)
           >> pairs_with_end:   many_till!(key_value_pair, line_ending)
           >> (Editor {..Default::default()})
          )
        );

named!(parse_osu<&str, Osu>, 
    do_parse!(
                    tag_s!("osu file format v")
    >>  version:    map_res!(digit, FromStr::from_str)
    >>              line_ending
    >>              opt!(multispace)
    >>  sections:   permutation!(call!(section_general), call!(section_editor))
    >>  (Osu {
        version: version,
        general: sections.0,
        })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../test.osu");
        let res = parse_osu(input);
        if let Done(_, osu) = res {
            println!("{:?}", osu);
            println!("{}", serde_json::to_string(&osu).unwrap());
        }
        else {
            println!("{:?}", res);
            assert!(false);
        }
    }
}
