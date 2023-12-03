mod day1;
mod day2;
mod day3;

use crate::day1::run_day1;

fn main() {
    println!("Advent of Code 2023!");

    run_day1();
    day2::run_day2();
    day3::run_day3();
}
