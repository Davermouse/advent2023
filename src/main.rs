mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use crate::day1::run_day1;

fn main() {
    println!("Advent of Code 2023!");

    run_day1();
    day2::run_day2();
    day3::run_day3();
    day4::run_day4();
    day5::run_day5();
}
