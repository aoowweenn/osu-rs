#[macro_use]
extern crate nom;

use nom::{digit, line_ending};
use nom::IResult::Done;
use nom::{space, anychar, crlf, multispace};

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
    pub fn new<'b>(pairs: Vec<(&'b str, &'b str)>) -> General<'a> {
        let mut map = HashMap::new();
        for (ref key, ref val) in pairs.clone() {
            map.insert(key.to_owned(), val.to_owned());
        }
        General{
            audio_filename: map.get("audio_filename").unwrap(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
struct Editor {
    asdf: u32,
}

named_args!(value<'a>(key: &'a str)<&'a [u8]>,
    do_parse!(
           opt!(multispace)
        >> tag_s!(key)
        >> opt!(space)
        >> char!(':')
        >> opt!(space)
        >> val: take_until_s!("\r\n")
        >> opt!(multispace)
        >> (val)
             )
    );

named!(key_value_pair<&str, (&str, &str)>,
    do_parse!(
                opt!(multispace)
        >> key: take_until_s!(":")
        >> val: take_until_s!("\r\n")
        >> opt!(multispace)
        >> (key, val)
    )
);

named!(section_general<&str, General>,
        do_parse!(
              opt!(multispace)
           >> tag_s!("[General]")
           >> opt!(multispace)
           >> pairs: many1!(key_value_pair)
           /*
        >> values: permutation!(
            apply!(value, "AudioFilename"),
            map_res!(apply!(value, "AudioLeadIn"), FromStr::from_str),
            map_res!(apply!(value, "PreviewTime"), FromStr::from_str),
            map_res!(apply!(value, "Countdown"), FromStr::from_str),
            apply!(value, "SampleSet"),
            map_res!(apply!(value, "StackLeniency"), FromStr::from_str),
            map_res!(apply!(value, "Mode"), FromStr::from_str),
            map_res!(apply!(value, "LetterboxInBreaks"), FromStr::from_str),
            map_res!(apply!(value, "WidescreenStoryboard"), FromStr::from_str),
            )
            */
        >> (General::new(pairs))
            /*
        >> (General {
            /*
            audio_filename: values.0,
            audio_lead_in: values.1,
            preview_time: values.2,
            countdown: values.3,
            sample_set: values.4,
            stack_leniency: values.5,
            mode: values.6,
            letterboxin_breaks: values.7,
            widescreen_storyboard: values.8,
            */
            ..Default::default()
            })
            */
          )
        );

named!(section_editor<&str, Editor>,
        do_parse!(
           tag_s!("[editor]")
          >> (Editor {..Default::default()})
          )
        );

named!(parse_osu<&str, Osu>, 
    do_parse!(
                    tag_s!("osu file format v")
    >>  version:    map_res!(digit, FromStr::from_str)
    >>              line_ending
    >>  sections:   permutation!(call!(section_general), call!(line_ending))
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
