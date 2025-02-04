// https://adventofcode.com/2024/day/2

use std::fs;

fn main() {
    // let contents: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let reports: Vec<Vec<isize>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace().map(|num| num.parse::<isize>().unwrap()).collect()
        })
        .collect();

    let safe: Vec<bool> = reports.iter().map(|report| {
        // determine if the report is sorted in ascending or descending order
        let s: bool = report.is_sorted() || report.is_sorted_by(|a, b| b <= a);

        // if so, ensure that the difference between each number is between 1 and 3
        if s { report.is_sorted_by(|a, b| {
            let diff = (a - b).abs();
            diff >=1 && diff <= 3
        })}
        else { false }
        }).collect();

    println!("{:?}", safe);

    let safe_count = safe.iter().filter(|&s| *s).count();
    println!("Safe reports: {}", safe_count);
}
