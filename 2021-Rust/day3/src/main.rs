use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines_strings(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}

fn position_is_one(num: &String, position: usize) -> bool {
    num.chars().nth(position).unwrap() == '1'
}

fn part1(input: &Vec<String>) -> i32 {
    let len = input.len();
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();
    for i in 0..input[0].len() {
        let ones = input
            .into_iter()
            .filter(|num| position_is_one(num, i))
            .count();

        gamma += &((ones > len - ones) as u8).to_string();
        epsilon += &(!(ones > len - ones) as u8).to_string();
    }

    i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsilon, 2).unwrap()
}

fn filter_most_common(input: Vec<String>, position: usize, is_gamma: bool) -> Vec<String> {
    let len = input.len();
    let ones = (&input)
        .into_iter()
        .filter(|num| position_is_one(num, position))
        .count();

    input
        .into_iter()
        .filter(|num| (is_gamma == position_is_one(num, position)) == (ones >= len - ones))
        .collect()
}

fn determine_rating(mut input: Vec<String>, is_gamma: bool) -> i32 {
    for i in 0..input[0].len() {
        input = filter_most_common(input, i, is_gamma);

        if input.len() == 1 {
            return i32::from_str_radix(&input[0], 2).unwrap();
        }
    }
    -1
}

fn part2(input: Vec<String>) -> i32 {
    determine_rating(input.clone(), true) * determine_rating(input, false)
}

fn main() {
    let input = read_lines_strings("./input");

    println!("{:?}", part1(&input));
    println!("{:?}", part2(input));
}
