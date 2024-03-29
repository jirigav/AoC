use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines_strings(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap()).collect()
}

fn part1(input: &Vec<String>) -> usize {
    input
        .into_iter()
        .map(|x| {
            let split: Vec<&str> = x.split("|").collect();
            split[1]
                .split_whitespace()
                .into_iter()
                .filter(|x| [2, 4, 3, 7].contains(&x.len()))
                .count()
        })
        .sum()
}

fn part2(input: Vec<String>) -> i64 {
    let mut one: HashSet<char> = HashSet::new();
    let mut four: HashSet<char> = HashSet::new();
    let mut result: i64 = 0;

    for line in input {
        for chars in line.split_whitespace().into_iter() {
            if chars == "|" {
                continue;
            }

            match chars.len() {
                2 => one = chars.chars().collect(),
                4 => four = chars.chars().collect(),
                _ => {}
            }
        }

        let to_resolve = &line.split("|").collect::<Vec<&str>>()[1];

        let mut output: i64 = 0;

        for chars in to_resolve.split_whitespace().into_iter() {
            match chars.len() {
                2 => output = output * 10 + 1,
                3 => output = output * 10 + 7,
                4 => output = output * 10 + 4,
                7 => output = output * 10 + 8,
                6 => {
                    let set: HashSet<char> = chars.chars().collect();
                    if set.difference(&four).count() == 2 {
                        output = output * 10 + 9;
                    } else if set.difference(&one).count() == 5 {
                        output = output * 10 + 6;
                    } else {
                        output = output * 10;
                    }
                }
                5 => {
                    let set: HashSet<char> = chars.chars().collect();
                    if set.difference(&one).count() == 3 {
                        output = output * 10 + 3;
                    } else if set.difference(&four).count() == 3 {
                        output = output * 10 + 2;
                    } else {
                        output = output * 10 + 5;
                    }
                }
                _ => {}
            }
        }
        result += output
    }

    result
}

fn main() {
    let input = read_lines_strings("./input");
    println!("{:?}", part1(&input));
    println!("{:?}", part2(input));
}
