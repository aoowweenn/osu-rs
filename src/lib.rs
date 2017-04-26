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
    metadata: Metadata,
    difficulty: Difficulty,
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

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Default, Serialize, StructMap)]
struct Metadata {
    title: String,
    title_unicode: String,
    artist: String,
    artist_unicode: String,
    creator: String,
    version: String,
    source: String,
    tags: Vec<String>,
    beatmap_ID: u32,
    beatmap_set_ID: u32,
}

#[derive(Debug, PartialEq, Default, Serialize, StructMap)]
struct Difficulty {
    hp_drain_rate: f32,
    circle_size: f32,
    overall_difficulty: f32,
    approach_rate: f32,
    slider_multiplier: f32,
    slider_tickrate: f32,
}

named!(key_value_pair<&str, (&str, &str)>,
    do_parse!(
           key: take_until_and_consume_s!(":")
        >> opt!(multispace)
        >> val: take_until_and_consume_s!("\r\n")
        >> (key, val)
    )
);

macro_rules! named_section {
    ( $func:ident<$T:ty> ) => (
named!($func<&str, $T>,
        do_parse!(
                                tag_s!(concat!("[", stringify!($T), "]"))
           >>                   opt!(multispace)
           >> pairs_with_end:   many_till!(key_value_pair, line_ending)
           >> (StructMap::from_tuples(pairs_with_end.0))
          )
        );
    )
}

named_section!(section_general<General>);
named_section!(section_editor<Editor>);
named_section!(section_metadata<Metadata>);
named_section!(section_difficulty<Difficulty>);

named!(parse_osu<&str, Osu>, 
    do_parse!(
                    tag_s!("osu file format v")
    >>  version:    map_res!(digit, FromStr::from_str)
    >>              line_ending
    >>              opt!(multispace)
    >>  sections:   permutation!(call!(section_general),
                                 call!(section_editor),
                                 call!(section_metadata),
                                 call!(section_difficulty))
    >>  (Osu {
        version:    version,
        general:    sections.0,
        editor:     sections.1,
        metadata:   sections.2,
        difficulty: sections.3,
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
