use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_first_line(file_path: &str) -> String {
    let file = File::open(file_path).expect("file wasn't found.");
    let mut reader = BufReader::new(file);
    let mut buffer: String = String::new();
    let _ = reader.read_line(&mut buffer);
    buffer.pop();
    buffer
}

fn simulate_days(mut array: [i64; 9], days: i32) -> [i64; 9] {
    for _ in 0..days {
        let zero = array[0];
        for i in 0..8 {
            array[i] = array[i + 1];
        }
        array[8] = zero;
        array[6] += zero;
    }
    array
}

fn main() {
    let mut array: [i64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let input = read_first_line("./input");

    for fish in input.split(",") {
        array[fish.parse::<usize>().unwrap()] += 1;
    }

    array = simulate_days(array, 80);

    println!("{:?}", IntoIterator::into_iter(array).sum::<i64>());

    array = simulate_days(array, 256 - 80);

    println!("{:?}", IntoIterator::into_iter(array).sum::<i64>());
}
