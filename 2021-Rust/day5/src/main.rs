use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_and_parse_lines(file_path: &str) -> Vec<((i32, i32), (i32, i32))> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .map(|x| {
            let split = x.split(" -> ").collect::<Vec<&str>>();
            let a = split[0].split(",").collect::<Vec<&str>>();
            let b = split[1].split(",").collect::<Vec<&str>>();
            (
                (a[0].parse::<i32>().unwrap(), a[1].parse::<i32>().unwrap()),
                (b[0].parse::<i32>().unwrap(), b[1].parse::<i32>().unwrap()),
            )
        })
        .collect()
}

fn add_point(points: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) {
    if points.contains_key(&(x, y)) {
        points.insert((x, y), points.get(&(x, y)).unwrap() + 1);
    } else {
        points.insert((x, y), 1);
    }
}

fn part1_and_2(input: &Vec<((i32, i32), (i32, i32))>, is_part2: bool) -> usize {
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    for ((x1, y1), (x2, y2)) in input {
        if x1 == x2 || y1 == y2 {
            for i in 0..((y2 - y1).abs() + 1) {
                for j in 0..((x2 - x1).abs() + 1) {
                    add_point(
                        &mut points,
                        if x1 > x2 { x2 + j } else { x2 - j },
                        if y1 > y2 { y2 + i } else { y2 - i },
                    );
                }
            }
        } else if is_part2 {
            for i in 0..((x1 - x2).abs() + 1) {
                add_point(
                    &mut points,
                    if x1 > x2 { x2 + i } else { x2 - i },
                    if y1 > y2 { y2 + i } else { y2 - i },
                );
            }
        }
    }
    points.values().into_iter().filter(|x| **x >= 2).count()
}

fn main() {
    let input = read_and_parse_lines("./input");
    println!("{:?}", part1_and_2(&input, false));
    println!("{:?}", part1_and_2(&input, true));
}
