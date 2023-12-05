use std::fs::File;
use std::io::Read;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{u64, alpha1, newline};
use nom::combinator::value;
use nom::multi::{separated_list1, many1};

#[derive(Debug)]
struct MapEntry {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    entries: Vec<MapEntry>
}

impl Map {
    fn transform(&self, val: u64) -> u64 {
        for entry in self.entries.iter() {
            if val > entry.source_start && val < entry.source_start + entry.length {
                return entry.dest_start + (val - entry.source_start);
            }
        }

        return val;
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn transform(&self, seed: u64) -> u64 {
        let mut current_type = "seed".to_string();
        let required_type = "location".to_string();
        let mut val = seed;

        while current_type != required_type {
            // Find the map for the current type
            let map = self.maps.iter().find(
                |m| m.from == current_type
            ).expect("Unable to find source map");

            val = map.transform(val);
            current_type = map.to.clone();
        }

        return val;
    }
}

fn parse_map_entry(i: &str) -> IResult<&str, MapEntry> {
    let (i, dest_start) = u64(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, source_start) = u64(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, length) = u64(i)?;

    Ok((i, MapEntry { dest_start: dest_start, source_start: source_start, length: length }))
}

fn parse_map(i: &str) -> IResult<&str, Map> {
    let (i, from) = alpha1(i)?;
    let (i, _) = tag("-to-")(i)?;
    let (i, to) = alpha1(i)?;
    let (i, _) = tag(" map:")(i)?;
    let (i, _) = newline(i)?;

    let (i, map_entries) = separated_list1(
        newline,
        parse_map_entry)(i)?;

    Ok((i, Map { from: from.to_string(), to: to.to_string() , entries: map_entries }))
}

fn parse_almanac(i: &str) -> IResult<&str, Almanac> {
    let (i, _) = tag("seeds: ")(i)?;
    let (i, seeds) = separated_list1(
        tag(" "),
        u64
    )(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = newline(i)?;

    let (i, maps) = 
        separated_list1(many1(newline), 
            parse_map)(i)?;

    Ok((i, Almanac { seeds: seeds, maps: maps}))
}

pub fn run_day5() {
    println!("Start day 5!");

    let mut f = File::open("data/day5.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let almanac = parse_almanac(&s).expect("Unable to parse almanac").1;

    println!("Loaded almanac: with {} maps", almanac.maps.len());

    let locations = almanac.seeds.iter().map(|s| almanac.transform(*s)).collect::<Vec<_>>();
    let lowest_location = locations.iter().min().expect("");
    println!("Lowest location {}", lowest_location);
}