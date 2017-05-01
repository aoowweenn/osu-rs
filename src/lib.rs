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
use nom::Needed;
use nom::IResult;
use nom::IResult::{Done, Incomplete};

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
    hit_objects: Vec<HitObj>,
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

#[derive(Debug, PartialEq, Default, Serialize, StructMap)]
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

#[derive(Debug, PartialEq, Default, Serialize, StructMap)]
struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq, Default, Serialize)]
struct HitObj {
    x: u32,
    y: u32,
    time: u32,
    ty: u32,
    hit_sound: u32,
    time_len: f32,
    time_end: u32,
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
           >>                   line_ending
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

named!(line_vec<&str, Vec<&str> >,
    do_parse!(
           v: separated_list!(char!(','), take_until_either!(",\r"))
        >>    line_ending
        >> (v)
    )
);

named!(section_events<&str, Events>,
    do_parse!(
                    tag_s!("[Events]")
        >>          line_ending
        >>          comment
        >> bg_ev:   line_vec
        >>          comment
        >> mov_ev:  line_vec
        >>          take_until_s!("[")
        >> (Events {
            background: if bg_ev.len() == 5 { bg_ev[2].trim_matches('"').to_owned() } else { "".to_owned() },
            video: if mov_ev.len() == 3 { mov_ev[2].trim_matches('"').to_owned() } else { "".to_owned() },
            ..Default::default()
        })
    )
);

named!(parse_timing_point<&str, TimingPoint>,
    do_parse!(
           v: line_vec
        >> (StructMap::from_vec(v))
    )
);

named!(section_timing_points<&str, Vec<TimingPoint> >,
    do_parse!(
                            tag_s!("[TimingPoints]")
        >>                  line_ending
        >> tps_with_end:    many_till!(parse_timing_point, line_ending)
        >>                  take_until_s!("[")
        >> (tps_with_end.0)
    )
);

named!(parse_colour<&str, Colour>,
    do_parse!(
           pair:    key_value_pair
        >> (StructMap::from_vec(pair.1.split(',').collect()))
    )
);

named!(section_colours<&str, Vec<Colour> >,
    do_parse!(
                            tag_s!("[Colours]")
        >>                  line_ending
        >> cs_with_end:     many_till!(parse_colour, line_ending)
        >>                  take_until_s!("[")
        >> (cs_with_end.0)
    )
);

fn parse_hit_obj(input: &str) -> IResult<&str, HitObj> {
    if let Done(offset, v) = line_vec(input) {
        let ret = match u32::from_str(v[3]).unwrap_or_default() {
            x if x & 0x1 != 0 => {
                HitObj {
                    x: FromStr::from_str(v[0]).unwrap(),
                    y: FromStr::from_str(v[1]).unwrap(),
                    time: FromStr::from_str(v[2]).unwrap(),
                    ty: FromStr::from_str(v[3]).unwrap(),
                    hit_sound: FromStr::from_str(v[4]).unwrap(),
                    ..Default::default()
                }
            }
            y if y & 0x2 != 0 => {
                HitObj {
                    x: FromStr::from_str(v[0]).unwrap(),
                    y: FromStr::from_str(v[1]).unwrap(),
                    time: FromStr::from_str(v[2]).unwrap(),
                    ty: FromStr::from_str(v[3]).unwrap(),
                    hit_sound: FromStr::from_str(v[4]).unwrap(),
                    time_len: FromStr::from_str(v[7]).unwrap(),
                    ..Default::default()
                }
            }
            z if z & 0x8 != 0 => {
                HitObj {
                    x: FromStr::from_str(v[0]).unwrap(),
                    y: FromStr::from_str(v[1]).unwrap(),
                    time: FromStr::from_str(v[2]).unwrap(),
                    ty: FromStr::from_str(v[3]).unwrap(),
                    hit_sound: FromStr::from_str(v[4]).unwrap(),
                    time_end: FromStr::from_str(v[5]).unwrap(),
                    ..Default::default()
                }
            }
            _ => HitObj { ..Default::default() },
        };
        Done(offset, ret)
    } else {
        Incomplete(Needed::Unknown)
    }
}

named!(section_hit_objects<&str, Vec<HitObj> >,
    do_parse!(
                    tag_s!("[HitObjects]")
        >>          line_ending
        >> hbs:     many1!(parse_hit_obj)
        >> (hbs)
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
                                 call!(section_colours),
                                 call!(section_hit_objects)
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
        hit_objects:    sections.7,
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
