#[macro_use]
extern crate nom;

use nom::{digit, line_ending};
use nom::IResult::Done;
//use nom::{space, alphanumeric, multispace};

use std::str::FromStr;

#[derive(Debug)]
#[repr(C)]
pub struct Osu {
    version: u32,
}

#[derive(Debug)]
struct General<'a> {
    audio_filename: &'a str,
    audio_lead_in:  u32,
    preview_time:   u32,
    countdown:      bool,
    sample_set:     &'a str,
    stack_leniency: f32,
    mode:           u32,
    letterboxin_breaks: bool,
    widescreen_Storyboard: bool,
}

//named!(section_general<&str, General>,

named!(parse_osu<&str, Osu>, 
    do_parse!(
                    tag_s!("osu file format v")
    >>  version:    map_res!(digit, FromStr::from_str)
    >>              line_ending
    //>>  general:    section_general
    >>  (Osu { version: version })
    )
);

#[no_mangle]
pub extern "C" fn parse_osu_file() -> *const Osu {
    //"Hello, world\0".as_ptr()
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
            println!("{:?}", osu)
        }
        else {
            assert!(false);
        }
    }
}
