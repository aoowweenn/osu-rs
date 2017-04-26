extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate nom;

mod traits;
use traits::{StructMap, FromStr};
#[macro_use]
extern crate struct_map_derive;

use nom::{digit, line_ending, multispace};
use nom::IResult::Done;

#[derive(Debug, Serialize)]
pub struct Osu {
    version: u32,
    general: General,
    editor: Editor,
}

#[derive(Debug, PartialEq, Serialize)]
enum Mode {
    Wrong = -1,
    Osu,
    Taiko,
    CatchTheBeat,
    OsuMania,
}

#[derive(Debug, PartialEq, Default, Serialize, StructMap)]
struct General {
    audio_filename: String,
    audio_lead_in: u32,
    preview_time: u32,
    countdown: bool,
    sample_set: String,
    stack_leniency: f32,
    mode: Mode,
    letterbox_in_breaks: bool,
    widescreen_storyboard: bool,
}

#[derive(Debug, PartialEq, Default, Serialize, StructMap)]
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
           >> (StructMap::from_tuples(pairs_with_end.0))
          )
        );

named!(section_editor<&str, Editor>,
        do_parse!(
                                tag_s!("[Editor]")
           >>                   opt!(multispace)
           >> pairs_with_end:   many_till!(key_value_pair, line_ending)
           >> (StructMap::from_tuples(pairs_with_end.0))
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
        editor:  sections.1,
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
        } else {
            println!("{:?}", res);
            assert!(false);
        }
    }
}
