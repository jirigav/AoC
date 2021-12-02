use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines_strings(file_path: &str) -> Vec<(String, i32)> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .map(|x| {
            (
                x.split(" ").collect::<Vec<&str>>()[0]
                    .parse::<String>()
                    .unwrap(),
                x.split(" ").collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap(),
            )
        })
        .collect()
}

fn count(input: &Vec<(String, i32)>, a: &str) -> i32 {
    input
        .into_iter()
        .filter(|(x, _y)| x == a)
        .map(|(_x, y)| y)
        .sum()
}

fn part1(input: &Vec<(String, i32)>) -> i32 {
    count(input, "forward") * (count(input, "down") - count(input, "up"))
}

fn part2(input: Vec<(String, i32)>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for i in input.iter() {
        match i {
            (x, y) if x == "down" => aim += y,
            (x, y) if x == "up" => aim -= y,
            (x, y) if x == "forward" => {
                horizontal += y;
                depth += aim * y
            }
            _ => println!("Failed to match value."),
        }
    }

    horizontal * depth
}

fn main() {
    let input = read_lines_strings("./input");
    println!("{:?}", part1(&input));
    println!("{:?}", part2(input));
}
