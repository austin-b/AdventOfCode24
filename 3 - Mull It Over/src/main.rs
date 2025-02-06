// https://adventofcode.com/2024/day/3

use regex::Regex;

fn main() {
    // let contents = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let contents = std::fs::read_to_string("src/input.txt").unwrap();

    // matches mul(X, Y)
    let re_mul: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    // matches numbers
    let re_nums: Regex = Regex::new(r"\d+").unwrap();

    // match to find all mul(X, Y) and then extract the numbers
    let matches  = re_mul.find_iter(&contents)
                                .map(|m| 
                                    // for each Match, extract the numbers with \d+
                                    re_nums.find_iter(m.as_str())
                                                                 .map(|n| 
                                                                    // for each number, parse it to i32
                                                                    n.as_str().parse::<i32>().unwrap()));

    let total = matches.fold(0, |acc, mut nums| {
        // multiply the two numbers and add to the accumulator
        acc + nums.next().unwrap() * nums.next().unwrap()
    });

    println!("Total: {}", total);
}
