use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines_strings(file_path: &str) -> (Vec<i32>, Vec<Vec<Vec<(i32, bool)>>>) {
    let file = File::open(file_path).expect("file wasn't found.");
    let mut reader = BufReader::new(file);

    let mut buf: String = String::new();
    let _ = reader.read_line(&mut buf);
    buf.pop();
    let bingo_values: Vec<i32> = buf
        .split(",")
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut i = 1;
    let mut bingos: Vec<Vec<Vec<(i32, bool)>>> = Vec::new();
    while i < lines.len() {
        if lines[i] == "" {
            i += 1;
            continue;
        } else {
            let mut bingo: Vec<Vec<(i32, bool)>> = Vec::new();
            for _ in 0..5 {
                bingo.push(
                    lines[i]
                        .split_whitespace()
                        .into_iter()
                        .map(|x| (x.parse::<i32>().unwrap(), false))
                        .collect(),
                );
                i += 1;
            }
            bingos.push(bingo);
        }
    }
    (bingo_values, bingos)
}

fn winning_move(bingo: &Vec<Vec<(i32, bool)>>, coordinates: (usize, usize)) -> bool {
    bingo
        .into_iter()
        .fold(true, |acc, x| acc & x[coordinates.1].1)
        || (&bingo[coordinates.0])
            .into_iter()
            .fold(true, |acc, x| acc & x.1)
}

fn mark_value(bingo: &mut Vec<Vec<(i32, bool)>>, value: i32) -> Option<(usize, usize)> {
    for i in 0..bingo.len() {
        for j in 0..bingo[0].len() {
            if bingo[i][j].0 == value {
                bingo[i][j].1 = true;
                return Some((i, j));
            }
        }
    }
    None
}

fn count_unmarked(bingo: &Vec<Vec<(i32, bool)>>) -> i32 {
    bingo
        .into_iter()
        .flatten()
        .filter(|x| !x.1)
        .map(|x| x.0)
        .sum()
}

fn part1(bingo_values: Vec<i32>, mut bingos: Vec<Vec<Vec<(i32, bool)>>>) -> Option<i32> {
    for value in bingo_values {
        for i in 0..bingos.len() {
            let coordinates = mark_value(&mut bingos[i], value);
            match coordinates {
                None => continue,
                Some((j, k)) => {
                    if winning_move(&bingos[i], (j, k)) {
                        return Some(count_unmarked(&bingos[i]) * value);
                    }
                }
            }
        }
    }
    None
}

fn part2(bingo_values: Vec<i32>, mut bingos: Vec<Vec<Vec<(i32, bool)>>>) -> Option<i32> {
    let mut finished: Vec<usize> = Vec::new();
    for value in bingo_values {
        for i in 0..bingos.len() {
            if finished.contains(&i) {
                continue;
            }
            let coordinates = mark_value(&mut bingos[i], value);
            match coordinates {
                None => continue,
                Some((j, k)) => {
                    if winning_move(&bingos[i], (j, k)) {
                        if !finished.contains(&i) {
                            finished.push(i);
                        }
                        if finished.len() == bingos.len() {
                            return Some(count_unmarked(&bingos[i]) * value);
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let (bingo_values, bingos) = read_lines_strings("./input");
    println!("{:?}", part1(bingo_values.clone(), bingos.clone()).unwrap());
    println!("{:?}", part2(bingo_values, bingos).unwrap());
}
