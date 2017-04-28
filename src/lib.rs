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
    events: Events,
    timing_points: Vec<TimingPoint>,
    colours: Vec<Colour>,
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

#[derive(Debug, PartialEq, Default, Serialize)]
struct Events {
    background: String,
    video: String,
    //break_periods: Vec<u32>,
}

#[derive(Debug, PartialEq, Default, Serialize)]
struct TimingPoint {
    offset: u32,
    msec_per_beat: f32,
    meter: u32,
    sample_type: u32,
    sample_set: u32,
    volume: u32,
    inherited: bool,
    kiai_mode: bool,
}

#[derive(Debug, PartialEq, Default, Serialize)]
struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

impl Colour {
    fn from_vec(v: Vec<u32>) -> Colour {
        Colour { red: v[0] as u8, green: v[1] as u8, blue: v[2] as u8 }
    }
}

named!(comment<&str, Vec<()> >,
    many0!(do_parse!(tag!("//") >> take_until_and_consume_s!("\r\n") >> ()))
);

named!(key_value_pair<&str, (&str, &str)>,
    do_parse!(
           key: take_until_and_consume_s!(":")
        >>      opt!(multispace)
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

named!(section_events<&str, Events>,
    do_parse!(
                    tag_s!("[Events]")
        >>          opt!(multispace)
        >>          comment
        >> bg_ev:   separated_list!(char!(','), take_until_either!(",\r"))
        >>          line_ending
        >>          comment
        >>          tag_s!("Video,")
        >> mov_ev:  separated_list!(char!(','), take_until_either!(",\r"))
        >>          line_ending
        >>          take_until_s!("[")
        >> (Events {
            background: if bg_ev.len() == 5 { bg_ev[2].trim_matches('"').to_owned() } else { "".to_owned() },
            video: if mov_ev.len() == 2 { mov_ev[1].trim_matches('"').to_owned() } else { "".to_owned() },
            ..Default::default()
        })
    )
);

named!(parse_timing_point<&str, TimingPoint>,
    do_parse!(
           line: separated_nonempty_list!(char!(','), take_until_either!(",\r"))
        >>       line_ending
        >> (TimingPoint {
                offset: FromStr::from_str(line[0]).unwrap(),
                msec_per_beat: FromStr::from_str(line[1]).unwrap(),
                meter: FromStr::from_str(line[2]).unwrap(),
                sample_type: FromStr::from_str(line[3]).unwrap(),
                sample_set: FromStr::from_str(line[4]).unwrap(),
                volume: FromStr::from_str(line[5]).unwrap(),
                inherited: u32::from_str(line[6]).unwrap() == 1,
                kiai_mode: u32::from_str(line[7]).unwrap() == 1,
        })
    )
);

named!(section_timing_points<&str, Vec<TimingPoint> >,
    do_parse!(
                            tag_s!("[TimingPoints]")
        >>                  opt!(multispace)
        >> tps_with_end:    many_till!(parse_timing_point, line_ending)
        >>                  take_until_s!("[")
        >> (tps_with_end.0)
    )
);

named!(parse_colour<&str, Colour>,
    do_parse!(
           pair:    key_value_pair
        >> (Colour::from_vec(FromStr::from_str(pair.1).unwrap()))
    )
);

named!(section_colours<&str, Vec<Colour> >,
    do_parse!(
                            tag_s!("[Colours]")
        >>                  opt!(multispace)
        >> cs_with_end:     many_till!(parse_colour, line_ending)
        >>                  take_until_s!("[")
        >> (cs_with_end.0)
    )
);

named!(parse_osu<&str, Osu>, 
    do_parse!(
                    tag_s!("osu file format v")
    >>  version:    map_res!(digit, FromStr::from_str)
    >>              line_ending
    >>              opt!(multispace)
    >>  sections:   permutation!(call!(section_general),
                                 call!(section_editor),
                                 call!(section_metadata),
                                 call!(section_difficulty),
                                 call!(section_events),
                                 call!(section_timing_points),
                                 call!(section_colours)
                                 )
    >>  (Osu {
        version:        version,
        general:        sections.0,
        editor:         sections.1,
        metadata:       sections.2,
        difficulty:     sections.3,
        events:         sections.4,
        timing_points:  sections.5,
        colours:        sections.6,
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
