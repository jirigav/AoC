use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines_nums(file_path: &str) -> Vec<i64> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect()
}

fn part1(values: &Vec<i64>) -> i64 {
    let mut count = 0;
    let mut prev_val = values[0];

    for v in values.iter() {
        if *v > prev_val {
            count += 1;
        }
        prev_val = *v;
    }
    count
}

fn part2(values: Vec<i64>) -> i64 {
    let mut new_vec: Vec<i64> = Vec::new();

    for i in 0..(values.len() - 2) {
        new_vec.push(values[i] + values[i + 1] + values[i + 2]);
    }
    part1(&new_vec)
}

fn main() {
    let values: Vec<i64> = read_lines_nums("./input");
    println!("{:?}", part1(&values));
    println!("{:?}", part2(values));
}
