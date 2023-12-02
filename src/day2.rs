use std::fs::File;
use std::io::Read;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::combinator::value;
use nom::multi::separated_list1;

#[derive(Debug,PartialEq)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn parse_color(i: &str) -> IResult<&str, (&str, u32)> {
    let (i, count) = u32(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, color) = alt((
        value("red", tag("red")),
        value("green", tag("green")),
        value("blue", tag("blue")),
    ))(i)?;

    Ok((i, (color, count)))
}

fn parse_round(i: &str) -> IResult<&str, Round> {
    let (i, colors) = separated_list1(
        tag(", "),
        parse_color
    )(i)?;

    let mut round = Round { red: 0, green: 0, blue: 0 };

    match colors.iter().find(|&x| x.0 == "red") {
        Some((_, c)) => round.red = *c,
        None => ()
    }

    match colors.iter().find(|&x| x.0 == "green") {
        Some((_, c)) => round.green = *c,
        None => ()
    }

    match colors.iter().find(|&x| x.0 == "blue") {
        Some((_, c)) => round.blue = *c,
        None => ()
    }

    Ok((i, round))
}

// Game 17: 5 blue; 1 blue; 2 red, 2 green, 4 blue; 6 blue, 4 green, 2 red
fn parse_game(i: &str) -> IResult<&str, Game> {
    let (i, _) = tag("Game ")(i)?;
    let (i, id) = u32(i)?;
    let (i, _) = tag(": ")(i)?;

    let (i, rounds) = separated_list1(
        tag("; "),
        parse_round
    )(i)?;

    Ok((i, Game { id: id, rounds: rounds}))
}

pub fn run_day2() {
    println!("Start day 2!");

    let mut f = File::open("data/day2.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let lines = s.lines().filter(|s| !s.is_empty()).collect::<Vec<_>>();

    println!("Loaded {} lines", lines.len());

    let games = lines.iter().map(|l| parse_game(l).expect("Unable to parse game").1).collect::<Vec<_>>();

    let valid_games = 
        games.iter()
            .filter(|g| 
                g.rounds.iter().all(|r| 
                    r.red <= 12 && r.green <= 13 && r.blue <= 14));
    
    let valid_id_total = valid_games.map(|g| g.id).sum::<u32>();

    println!("Valid id total: {}", valid_id_total);

    let game_powers =
                    games.iter()
                        .map(|g| {
                            let max_r = g.rounds.iter().map(|r| r.red).max().expect("");
                            let max_g = g.rounds.iter().map(|r| r.green).max().expect("");
                            let max_b = g.rounds.iter().map(|r| r.blue).max().expect("");

                            max_r * max_b * max_g
                        }).sum::<u32>();

    print!("Miniumum powers: {}", game_powers);
}