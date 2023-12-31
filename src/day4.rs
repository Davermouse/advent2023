use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

use nom::IResult;
use nom::character::complete::multispace1;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;
use nom::character::complete::i32;

struct Card {
    winning: HashSet<i32>,
    numbers: HashSet<i32>,
}

fn compute_score(c: &Card) -> usize {
    return c.winning.iter().filter(|w| c.numbers.contains(w)).count();
}

// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
fn parse_card(i: &str) -> IResult<&str, Card> {
    let (i, _) = tag("Card")(i)?;
    let (i, _) = multispace1(i)?;
    let (i, id) = i32(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = multispace1(i)?;

    let (i, winning) = separated_list1(
        multispace1, 
        i32)(i)?;
    let (i, _) = tag(" |")(i)?;
    let (i, _) = multispace1(i)?;
    let (i, numbers) = separated_list1(
        multispace1, 
        i32)(i)?;

    Ok((i, Card {
        winning: HashSet::from_iter(winning.into_iter()), 
        numbers: HashSet::from_iter(numbers.into_iter()) 
    }))
}

pub fn run_day4() {
    println!("Day 1!");

    let mut f = File::open("data/day4.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let lines = s.lines().filter(|s| !s.is_empty()).collect::<Vec<_>>();

    println!("Loaded {} lines", lines.len());

    let cards = lines.iter().map(|l| parse_card(l).expect("Unable to parse card").1).collect::<Vec<_>>();

    let score: u32 = cards.iter().map(
        |c| {
        let score = compute_score(c);

        if score == 0 {
            return 0;
        } else {
            return 2_u32.pow((score - 1).try_into().unwrap());
        }
    }).sum();

    println!("Part 1: {}", score);

    let mut copies = vec!(1 ; cards.len());

    for i in 0..cards.len() {
        let card = &cards[i];
        let amount = copies[i];
        let score = compute_score(card);

        for n in (i+1)..(i+1+score) {
            copies[n] += amount;
        }
    }

    let total_cards: i32 = copies.iter().sum();

    println!("Part 2: {}", total_cards);
}