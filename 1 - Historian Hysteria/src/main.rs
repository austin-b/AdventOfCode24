// https://adventofcode.com/2024/day/1

use std::fs;

fn main() {
    //let contents: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let pairs: Vec<(isize, isize)> = contents
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|num| num.parse::<isize>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();

    let mut left: Vec<isize> = pairs.iter().map(|pair| pair.0).collect();
    let mut right: Vec<isize> = pairs.iter().map(|pair| pair.1).collect();

    left.sort();
    right.sort();

    let distance: isize = left.iter().zip(right.iter())
                                     .map(|(l, r)| (l - r).abs())
                                     .sum();

    println!("{}", distance);
}
