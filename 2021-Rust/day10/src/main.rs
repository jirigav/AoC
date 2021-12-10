use std::collections::HashMap;
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

fn count_score1(counter: HashMap<char, i64>) -> i64 {
    let mut points: HashMap<char, i64> = HashMap::new();
    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1197);
    points.insert('>', 25137);
    let mut score: i64 = 0;

    for k in counter.keys() {
        score += counter.get(k).unwrap() * points.get(k).unwrap();
    }
    score
}

fn count_score2(mut stack: Vec<char>) -> i64 {
    let mut points: HashMap<char, i64> = HashMap::new();
    points.insert(')', 1);
    points.insert(']', 2);
    points.insert('}', 3);
    points.insert('>', 4);
    let mut score: i64 = 0;

    while !stack.is_empty() {
        let found = stack.pop().unwrap();
        score = (score * 5) + points.get(&found).unwrap();
    }
    score
}

fn solve(input: &Vec<String>, is_part2: bool) -> i64 {
    let mut counter: HashMap<char, i64> = HashMap::new();
    let mut scores: Vec<i64> = Vec::new();
    counter.insert(')', 0);
    counter.insert(']', 0);
    counter.insert('}', 0);
    counter.insert('>', 0);

    for line in input {
        let mut incorect = false;
        let mut stack: Vec<char> = Vec::new();
        for chr in line.chars() {
            match chr {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                a => {
                    if stack.pop().unwrap() != a {
                        if !is_part2 {
                            counter.insert(a, counter.get(&a).unwrap() + 1);
                        }
                        incorect = true;
                        break;
                    }
                }
            }
        }
        if !incorect && is_part2 {
            scores.push(count_score2(stack));
        }
    }
    if is_part2 {
        scores.sort();
        return scores[scores.len() / 2];
    }
    count_score1(counter)
}

fn main() {
    let input = read_lines_strings("./input");
    println!("{:?}", solve(&input, false));
    println!("{:?}", solve(&input, true));
}
