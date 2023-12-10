use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Index;

use nom::{IResult, InputIter};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, u32, alphanumeric1};
use nom::combinator::value;
use nom::multi::{separated_list1, many1};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_handtype(cards: &HashMap<char, u8>) -> HandType {
    // Five of a kind, where all five cards have the same label: AAAAA
    if cards.iter().any(|e| *e.1 == 5) {
        return HandType::FiveOfAKind;
    }

    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    if cards.iter().any(|e| *e.1 == 4) {
        return HandType::FourOfAKind;
    }

    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    if cards.iter().any(|e| *e.1 == 3) &&
       cards.iter().any(|e| *e.1 == 2) {
        return HandType::FullHouse;
    }

    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    if cards.iter().any(|e| *e.1 == 3) &&
       cards.iter().any(|e| *e.1 == 1) {
        return HandType::ThreeOfAKind;
    }

    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    if cards.iter().filter(|e| *e.1 == 2).count() == 2 {
        return HandType::TwoPair;
    }

    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    if cards.iter().filter(|e| *e.1 == 2).count() == 1 &&
       cards.iter().filter(|e| *e.1 == 1).count() == 3 {
        return HandType::OnePair;
    }

    // High card, where all cards' labels are distinct: 23456
    return HandType::HighCard;
}

fn get_handtype_part_2(cards: &mut HashMap<char, u8>) -> HandType {

    // Five of a kind, where all five cards have the same label: AAAAA
    if cards.iter().any(|e| *e.1 == 5) {
        return HandType::FiveOfAKind;
    }

    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    if cards.iter().any(|e| *e.1 == 4) {
        return HandType::FourOfAKind;
    }

    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    if cards.iter().any(|e| *e.1 == 3) &&
       cards.iter().any(|e| *e.1 == 2) {
        return HandType::FullHouse;
    }

    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    if cards.iter().any(|e| *e.1 == 3) &&
       cards.iter().any(|e| *e.1 == 1) {
        return HandType::ThreeOfAKind;
    }

    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    if cards.iter().filter(|e| *e.1 == 2).count() == 2 {
        return HandType::TwoPair;
    }

    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    if cards.iter().filter(|e| *e.1 == 2).count() == 1 &&
       cards.iter().filter(|e| *e.1 == 1).count() == 3 {
        return HandType::OnePair;
    }

    // High card, where all cards' labels are distinct: 23456
    return HandType::HighCard;
}

const HANDTYPE_RANKINGS: [HandType ; 7] = [
    HandType::FiveOfAKind, HandType::FourOfAKind, HandType::FullHouse, HandType::ThreeOfAKind, HandType::TwoPair, HandType::OnePair, HandType::HighCard];

const CARD_ORDER: &'static str = "AKQJT98765432";
const CARD_ORDER_PT_2: &'static str = "AKQT98765432J";

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    card_counts: HashMap<char, u8>,
    bid: u32,
    hand_type: HandType
}

fn parse_hand(i: &str) -> IResult<&str, Hand> {
    let (i, card_chars) = alphanumeric1(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, bid) = u32(i)?;

    let cards = card_chars.chars().collect::<Vec<_>>();
    let mut card_counts = HashMap::new();

    for card in cards.iter() {
        if card_counts.contains_key(card) {
            let mut current_count = card_counts.get_mut(card).expect("");
            *current_count += 1;
        } else {
            card_counts.insert(card.clone(), 1);
        }
    }

    let hand_type = get_handtype(&card_counts);

    Ok((i, Hand { cards: cards, card_counts: card_counts, bid: bid, hand_type: hand_type }))
}

fn parse_hand_pt2(i: &str) -> IResult<&str, Hand> {
    let (i, card_chars) = alphanumeric1(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, bid) = u32(i)?;

    let cards = card_chars.chars().collect::<Vec<_>>();
    let mut card_counts = HashMap::new();

    for card in cards.iter() {
        if card_counts.contains_key(card) {
            let mut current_count = card_counts.get_mut(card).expect("");
            *current_count += 1;
        } else {
            card_counts.insert(card.clone(), 1);
        }
    }

    let joker_count = card_counts.get(&'J').unwrap_or(&0).clone();

    let highest_card = card_counts.iter_mut().max_by(|x, y| x.1.cmp(&y.1)).expect("");
    *highest_card.1 = *highest_card.1 + joker_count;

    let hand_type = get_handtype(&mut card_counts);

    Ok((i, Hand { cards: cards, card_counts: card_counts, bid: bid, hand_type: hand_type }))
}

fn order_hands(hands: &mut Vec<Hand>, card_order: &str) {
    hands.sort_by(|a, b| {
        let ah = a.hand_type as isize;
        let bh = b.hand_type as isize;

        if ah != bh {
            return ah.cmp(&bh);
        }

        let pairs = a.cards.iter().zip(b.cards.iter());

        for (ca, cb) in pairs {
            if *ca != *cb {
                return card_order.position(|e| e == *ca)
                    .expect("")
                    .cmp(&card_order.position(|e| e == *cb).expect(""));
            }
        }

        return Ordering::Equal;
    });
}

fn compute_part_1_score(mut hands: &Vec<Hand>) -> u32 {
    (0..hands.len()).map(|i| hands[i].bid * (i + 1) as u32).sum::<u32>()
}

pub fn run_day7() {
    println!("Start day 7!");

    let mut f = File::open("data/day7.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let mut hands = 
        s.lines().map(|l| 
            parse_hand(l).expect("Unable to parse hand").1
    ).collect::<Vec<_>>();

    order_hands(&mut hands, CARD_ORDER);
    hands.reverse();

    let summed_score = compute_part_1_score(&hands);
    println!("Part 1 score {}", summed_score);

    let mut hands_2 = 
    s.lines().map(|l| 
        parse_hand_pt2(l).expect("Unable to parse hand").1
    ).collect::<Vec<_>>();

    order_hands(&mut hands_2, CARD_ORDER_PT_2);
    hands_2.reverse();

    let summed_score_2 = compute_part_1_score(&hands_2);
    println!("Part 2 score {}", summed_score_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranking() {
        let mut f = File::open("data/test_day7.txt").expect("File not found");
        let mut s = String::new();
        f.read_to_string(&mut s).expect("Unable to load file");
    
        let mut hands = 
            s.lines().map(|l| 
                parse_hand(l).expect("Unable to parse hand").1
        ).collect::<Vec<_>>();

        order_hands(&mut hands, CARD_ORDER);
        hands.reverse();

        println!("{:?}", hands);

        let score = compute_part_1_score(&hands);

        assert_eq!(score, 6440);
    }

    #[test]
    fn test_part_2() {
        let mut f = File::open("data/test_day7.txt").expect("File not found");
        let mut s = String::new();
        f.read_to_string(&mut s).expect("Unable to load file");
    
        let mut hands = 
            s.lines().map(|l| 
                parse_hand_pt2(l).expect("Unable to parse hand").1
        ).collect::<Vec<_>>();

        order_hands(&mut hands, CARD_ORDER_PT_2);
        hands.reverse();

        println!("{:?}", hands);

        let score = compute_part_1_score(&hands);

        assert_eq!(score, 5905);
    }
}