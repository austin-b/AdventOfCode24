// https://adventofcode.com/2024/day/1

use std::fs;
use std::collections::HashMap;

fn main() {
    // let contents: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    let contents: String = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    let pairs: Vec<(isize, isize)> = contents
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|num| num.parse::<isize>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();

    let left: Vec<isize> = pairs.iter().map(|pair| pair.0).collect();
    let right: Vec<isize> = pairs.iter().map(|pair| pair.1).collect();

    // left.sort();
    // right.sort();

    // count right list by using a HashMap
    let mut right_count: HashMap<isize, isize> = HashMap::new();
    for &n in &right {
        *right_count.entry(n).or_insert(0) += 1;
    }

    let mut similarity = 0;

    left.iter().for_each(|n| {
        let count = right_count.get(n).unwrap_or(&0);
        // println!("{}: {} = score: {}", n, count, n * count);
        similarity += n * count;
    });

    println!("Similarity: {}", similarity);
}
