#[macro_use]
extern crate nom;

use nom::{digit, line_ending};
use nom::IResult::Done;
use nom::{space, multispace};

use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
#[repr(C)]
pub struct Osu<'a> {
    version: u32,
    general: General<'a>,
}

#[derive(Debug, PartialEq, Default)]
struct General<'a> {
    audio_filename: &'a str,
    audio_lead_in:  u32,
    preview_time:   u32,
    countdown:      bool,
    sample_set:     &'a str,
    stack_leniency: f32,
    mode:           u32,
    letterboxin_breaks: bool,
    widescreen_storyboard: bool,
}

impl<'a> General<'a> {
    pub fn new(pairs: Vec<(&'a str, &'a str)>) -> General<'a> {
        let mut map = HashMap::new();
        for (ref key, ref val) in pairs {
            map.insert(key.to_owned(), val.to_owned());
        }
        let unwrap_get = |x| map.get(x).unwrap();
        let unwrap_from_str = |x| FromStr::from_str(unwrap_get(x)).unwrap();
        General{
            audio_filename: unwrap_get("AudioFilename"),
            audio_lead_in: unwrap_from_str("AudioLeadIn"),
            mode: unwrap_from_str("Mode"),
            ..Default::default()
        }
    }
}

#[derive(Debug, PartialEq, Default)]
struct Editor {
    asdf: u32,
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
           >> (General::new(pairs_with_end.0))
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

#[no_mangle]
pub extern "C" fn parse_osu_file<'a>() -> *const Osu<'a> {
    let input = include_str!("../test.osu");
    let res = parse_osu(input);
    match res {
        Done(_, x) => return &x,
        _ => return 0 as *const Osu,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../test.osu");
        let res = parse_osu(input);
        if let Done(_, osu) = res {
            println!("{:?}", osu);
        }
        else {
            println!("{:?}", res);
            assert!(false);
        }
    }
}
