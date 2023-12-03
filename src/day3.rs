use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

use derivative::Derivative;

#[derive(Debug, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
enum Cell {
    Empty,
    Symbol(char),
    Digit(
        #[derivative(PartialEq="ignore")]
        #[derivative(Hash="ignore")]
        u32, 
        #[derivative(PartialEq="ignore")]
        #[derivative(Hash="ignore")]
        usize, 
        u32)
}

fn char_to_cell(c: char) -> Cell {
    if c == '.' {
        return Cell::Empty;
    } else if c.is_numeric() {
        return Cell::Digit(c.to_digit(10).expect("Unable to parse number as digit"), 0, 0);
    } else {
        return Cell::Symbol(c);
    }
}

pub fn run_day3() {
    println!("Start day 3!");

    let mut f = File::open("data/day3.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let lines = s.lines().filter(|s| !s.is_empty()).collect::<Vec<_>>();

    let width = lines[0].len();
    let height = lines.len();

    let mut engine = lines.iter().map(|line| {
        line.chars().map(char_to_cell).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut number_id = 0;

    for y in 0..height {
        let mut x = 0;

        while x < width {
            let c = &engine[y][x];

            match c {
                Cell::Empty => (),
                Cell::Symbol(_) => (),
                Cell::Digit(_, _, _) => {
                    // Find the width and value of the number
                    let mut w = 0;
                    let mut num = 0;

                    for dx in x..width {
                        match engine[y][dx] {
                            Cell::Empty => break,
                            Cell::Symbol(_) => break,
                            Cell::Digit(n, _, _) => {
                                num *= 10;
                                num += n;
                                w += 1;
                            }
                        }
                    }

                    for dx in x..(x + w) {
                        engine[y][dx] = Cell::Digit(num, w, number_id)
                    }

                    number_id += 1;
                    x += w - 1;
                }
            }

            x += 1;
        }
    }

    let mut part_number_total = 0;

    for y in 0..height {
        let mut x = 0;

        while x < width {
            let c = &engine[y][x];

            match c {
                Cell::Empty => (),
                Cell::Symbol(_) => (),
                Cell::Digit(val, w, _) => {
                    // Check if there is a part adjacent to the number
                    let mut has_part = false;
                    let skip = if x == 0 { x } else { x - 1 };
                    if y != 0 {
                        has_part |= engine[y - 1].iter().skip(skip).take(w + 2).any(|c| matches!(c, Cell::Symbol(_)));
                    }
                    has_part |= engine[y].iter().skip(skip).take(w + 2).any(|c| matches!(c, Cell::Symbol(_)));

                    if y != height - 1 {
                        has_part |= engine[y + 1].iter().skip(skip).take(w + 2).any(|c| matches!(c, Cell::Symbol(_)));
                    }

                    if has_part {
                        part_number_total += val;
                    }

                    x += w - 1;
                }
            }

            x += 1;
        }
    }

    println!("Part one result: {}", part_number_total);

    if part_number_total != 550064 {
        panic!("Incorrect value for Day 3 Part 1");
    }

    let mut ratio_total = 0;

    for y in 0..height {
        let mut x = 0;

        for x in 0..width {
            let c = &engine[y][x];

            match c {
                Cell::Symbol('*') => {
                    let mut adjacent_cells = HashSet::new();
                    let skip = if x == 0 { x } else { x - 1 };

                    if y != 0 {
                        adjacent_cells.extend(engine[y - 1].iter().skip(skip).take(3));
                    }
                    adjacent_cells.extend(engine[y].iter().skip(skip).take(3));
                    if y != height - 1 {
                        adjacent_cells.extend(engine[y + 1].iter().skip(skip).take(3));
                    }

                    let adjacent_numbers = adjacent_cells.iter().filter(|c| matches!(c, Cell::Digit(_, _, _))).collect::<Vec<_>>();

                    if adjacent_numbers.len() == 2 {
                        if let Cell::Digit(a, _, _) = adjacent_numbers[0] {
                            if let Cell::Digit(b, _, _) = adjacent_numbers[1] {
                                ratio_total += a * b;
                            }
                        }
                    }
                },
                Cell::Empty | Cell::Digit(_, _, _) | Cell::Symbol(_) => ()
            }
        }
    }

    println!("Part two result: {}", ratio_total);
}