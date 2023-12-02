use std::fs::File;
use std::io::Read;

pub fn run_day1() {
    println!("Day 1!");

    let mut f = File::open("data/day1.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let lines = s.lines().filter(|s| !s.is_empty()).collect::<Vec<_>>();

    println!("Loaded {} lines", lines.len());

    let numeric_lines = lines.iter().map(|l| {
        l.chars().filter(|c| c.is_numeric()).collect::<String>()
    }).collect::<Vec<_>>();

    let total = numeric_lines.iter().map(|l| {
        let mut cs = l.chars().collect::<Vec<_>>();
        let f = cs[0];
        cs.reverse();
        let e = cs[0];

        return f.to_digit(10).expect("First is not a digit") * 10 + e.to_digit(10).expect("Last is not a digit");
    }).sum::<u32>();

    println!("Part 1 total: {}", total);

    let numeric_spelled_lines = 
        lines.iter()
            .map(|l| 
                convert_numbers(l.to_string()).chars().filter(|c| c.is_numeric()).collect::<String>()
            ).collect::<Vec<_>>();

    let spelled_total = numeric_spelled_lines.iter().map(|l| {
        let mut cs = l.chars().collect::<Vec<_>>();
        let f = cs[0];
        cs.reverse();
        let e = cs[0];

        return f.to_digit(10).expect("First is not a digit") * 10 + e.to_digit(10).expect("Last is not a digit");
    }).sum::<u32>();

    println!("Part 2 total: {}", spelled_total);

}

fn convert_numbers(s: String) -> String {
    return s.replace("one", "o1e")
    .replace("two", "t2o")
    .replace("three", "t3e")
    .replace("four", "f4r")
    .replace("five", "f5e")
    .replace("six", "s6x")
    .replace("seven", "s7n")
    .replace("eight", "e8t")
    .replace("nine", "n9e")
    .chars().filter(|c| c.is_numeric()).collect::<String>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_numbers() {
        assert_eq!(convert_numbers("5four1bvggfs62nineone".to_string()), "5416291");
        assert_eq!(convert_numbers("two1nine".to_string()), "219");
        assert_eq!(convert_numbers("xtwone3four".to_string()), "2134");
        assert_eq!(convert_numbers("zoneight234".to_string()), "18234");
    }
}