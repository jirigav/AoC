#![feature(mixed_integer_ops)]
use priority_queue::PriorityQueue;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(file_path: &str) -> Vec<Vec<i64>> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|x| i64::from_str_radix(&x.to_string(), 10).unwrap())
                .collect()
        })
        .collect()
}

fn get_neighbours(i: usize, j: usize, len: usize) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    for (a, b) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        match (i.checked_add_signed(a), j.checked_add_signed(b)) {
            (Some(k), Some(l)) => {
                if k < len && l < len {
                    neighbours.push((k, l))
                }
            }
            _ => {}
        }
    }
    neighbours
}

fn dijkstra(input: Vec<Vec<i64>>) -> i64 {
    let mut queue: PriorityQueue<(usize, usize), i64> = PriorityQueue::new();
    for i in 0..input.len() {
        for j in 0..input.len() {
            queue.push((i, j), -(i64::max_value() / 2));
        }
    }

    queue.change_priority(&(0, 0), 0);

    loop {
        let ((i, j), val) = queue.pop().unwrap();

        if i == (input.len() - 1) && j == (input[0].len() - 1) {
            return -val;
        }

        for (a, b) in get_neighbours(i, j, input.len()).into_iter() {
            match queue.get_priority(&(a, b)) {
                Some(x) => {
                    if *x < val - input[a][b] {
                        queue.change_priority(&(a, b), val - input[a][b]);
                    }
                }
                None => {}
            }
        }
    }
}

fn add_max9(x: i64, m: i64) -> i64 {
    let mut a = x + m;
    if a > 9 {
        a -= 9
    }
    a
}

fn modify_input(mut input: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let len = input.len();
    for m in 1..5 {
        for i in 0..len {
            input.push(
                input[i]
                    .clone()
                    .into_iter()
                    .map(|x| add_max9(x, m))
                    .collect(),
            )
        }
    }

    for m in 1..5 {
        for v in &mut input {
            for i in 0..len {
                v.push(add_max9(v[i], m));
            }
        }
    }
    input
}

fn main() {
    let input = parse("./input");

    println!("{:?}", dijkstra(input.clone()));
    println!("{:?}", dijkstra(modify_input(input)));
}
