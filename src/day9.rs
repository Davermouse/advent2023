use std::fs::File;
use std::io::Read;

fn extrapolate(seq: &Vec<i32>) -> i32 {
    let diffs = 
        seq.iter()
            .zip(seq.iter().skip(1))
            .map(|a| a.1 - a.0)
            .collect::<Vec<_>>();

    if diffs.iter().all(|d| *d == 0) {
        return *seq.iter().last().expect("");
    } else {
        return seq.iter().last().expect("") + extrapolate(&diffs);
    }
}

fn extrapolate_beginning(seq: &Vec<i32>) -> i32 {
     let diffs = 
         seq.iter()
             .zip(seq.iter().skip(1))
             .map(|a| a.1 - a.0)
             .collect::<Vec<_>>();
 
     if diffs.iter().all(|d| *d == 0) {
         return seq[0];
     } else {
         return seq[0] - extrapolate_beginning(&diffs);
     }
 }

pub fn run_day9() {
    println!("Run day 9!");

    let mut f = File::open("data/day9.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let seqs = 
        s.lines()
            .map(|l| 
                l.split(" ").map(|n| 
                    n.parse::<i32>().expect("")).collect::<Vec<_>>()
                ).collect::<Vec<_>>();

    let extrapolated = seqs.iter().map(|s| extrapolate(s)).sum::<i32>();

    println!("Part 1: {}", extrapolated);

    let extrapolated_beginnings = seqs.iter().map(|s| extrapolate_beginning(s)).sum::<i32>();

    println!("Part 2: {}", extrapolated_beginnings);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(&vec!(1, 2, 3)), 4);
        assert_eq!(extrapolate(&vec!(1, 3, 6, 10, 15, 21)), 28);
    }

    #[test]
    fn test_extrapolate_beginning() {
        assert_eq!(extrapolate_beginning(&vec!(1, 2, 3)), 0);
        assert_eq!(extrapolate_beginning(&vec!(10, 13, 16, 21, 30, 45)), 5);
    }
}