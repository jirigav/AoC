use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_first_line_numbers(file_path: &str) -> Vec<i64> {
    let file = File::open(file_path).expect("file wasn't found.");
    let mut reader = BufReader::new(file);
    let mut buffer: String = String::new();
    let _ = reader.read_line(&mut buffer);
    buffer.pop();
    buffer
        .split(",")
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn solve(input: &Vec<i64>, f: fn(&i64) -> i64) -> i64 {
    let mut min: i64 = input.into_iter().map(f).sum();

    for i in *input.into_iter().min().unwrap()..*input.into_iter().max().unwrap() {
        let min2 = input.into_iter().map(|x| f(&(x - i).abs())).sum();
        if min > min2 {
            min = min2;
        }
    }
    min
}

fn main() {
    let input = read_first_line_numbers("./input");

    println!("{:?}", solve(&input, |x| *x));
    println!(
        "{:?}",
        solve(&input, |a| (*a as f64 * ((a + 1) as f64 / 2.0)) as i64)
    );
}
