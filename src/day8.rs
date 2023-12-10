use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use num::Integer;

use nom::{IResult, character::complete::{alphanumeric1, newline}, multi::{separated_list1, many1}, bytes::complete::tag, sequence::separated_pair};

#[derive(Debug)]
struct Map {
    moves: Vec<char>,

    nodes: HashMap<String, (String, String)>
}


fn parse_map(i: &str) -> IResult<&str, Map> {
    let (i, raw_moves) = alphanumeric1(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = newline(i)?;

    let (i, nodes) = separated_list1(
        many1(newline),
        |i| {
            let (i, id) = alphanumeric1(i)?;
            let (i, _) = tag(" = (")(i)?;
            let (i, lr) = separated_pair(alphanumeric1, tag(", "), alphanumeric1)(i)?;
            let (i, _) = tag(")")(i)?;

            return Ok((i, (id, lr)));
        })(i)?;

    let mut map = HashMap::new();

    for node in nodes.iter() {
        map.insert(node.0.to_string(), (node.1.0.to_string(), node.1.1.to_string()));
    }

    return Ok((i, Map { moves: raw_moves.chars().collect::<Vec<_>>(), nodes: map }));
}

fn compute_length_part_1(map: &Map, from: String, to: String) -> usize {
    let mut move_count = 0;
    let mut current_node = from;

    while current_node != to {
        let node = map.nodes.get(&current_node).expect("Unable to find node");
        let m = map.moves.get(move_count % map.moves.len()).expect("");

        if *m == 'L' {
            current_node = node.0.clone();
        } else {
            current_node = node.1.clone();
        }

        move_count += 1;
    }

    move_count
}

fn compute_length_part_2(map: &Map, from: String) -> usize {
    let mut move_count = 0;
    let mut current_node = from;

    while !current_node.ends_with("Z") {
        let node = map.nodes.get(&current_node).expect("Unable to find node");
        let m = map.moves.get(move_count % map.moves.len()).expect("");

        if *m == 'L' {
            current_node = node.0.clone();
        } else {
            current_node = node.1.clone();
        }

        move_count += 1;
    }

    move_count
}

pub fn run_day8() {
    println!("Start day 8!");

    let mut f = File::open("data/day8.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let map = parse_map(&s).expect("Unable to parse map").1;

    let mut current_node = "AAA".to_string();
    let destination_node = "ZZZ".to_string();
   
    let part_1_length = 
        compute_length_part_1(
            &map, 
            "AAA".to_string(),
            "ZZZ".to_string()
        );

    println!("Part one result: {}", part_1_length);

    if part_1_length != 22411 {
        panic!("Unexpected result for day 8 part 1");
    }

    let mut current_nodes = 
        map.nodes.iter()
            .filter(
                |n| 
                    n.0.ends_with("A"))
            .map(|n| 
                compute_length_part_2(&map, n.0.clone()) as i32).collect::<Vec<_>>();

  //  let part_2_length = current_nodes.into_iter().reduce(|acc, steps|  acc.lcm(&steps)).unwrap();

  //  println!("Part two result: {}", part_2_length);


/*
    move_count = 0;
    move_index = 0;
    

    while !current_nodes.iter().all(|n| n.ends_with("Z")) {
        let m = map.moves.get(move_index).expect("");

        current_nodes = current_nodes.iter().map(|current_node| {
            let node = map.nodes.get(current_node).expect("Unable to find node");

            if *m == 'L' {
                return node.0.clone();
            } else {
                return node.1.clone();
            }
        }).collect();

        move_count += 1;
        move_index += 1;

        if move_index == map.moves.len() {
            move_index = 0;
        }
    }

    println!("Part two result: {}", move_count);
    */

}