use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines_to_map(file_path: &str) -> (HashSet<(u32, u32)>, Vec<(char, u32)>) {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut dots: HashSet<(u32, u32)> = HashSet::new();
    let mut instructions: Vec<(char, u32)> = Vec::new();
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut i = false;
    for line in lines {
        if line == "" {
            i = true;
        } else if i {
            let chr = line.chars().nth(11).unwrap();
            let num: u32 = line[13..].parse::<u32>().unwrap();
            instructions.push((chr, num));
        } else {
            let split = line
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            dots.insert((split[0], split[1]));
        }
    }
    (dots, instructions)
}

fn fold(
    dots: &HashSet<(u32, u32)>,
    instruction: (char, u32),
) -> (HashSet<(u32, u32)>, HashSet<(u32, u32)>) {
    let mut to_remove: HashSet<(u32, u32)> = HashSet::new();
    let mut to_add: HashSet<(u32, u32)> = HashSet::new();

    for dot in (&dots).into_iter() {
        if instruction.0 == 'x' {
            if dot.0 > instruction.1 {
                to_remove.insert(dot.clone());
                to_add.insert((2 * instruction.1 - dot.0, dot.1));
            }
            if dot.0 == instruction.1 {
                to_remove.insert(dot.clone());
            }
        } else {
            if dot.1 > instruction.1 {
                to_remove.insert(dot.clone());
                to_add.insert((dot.0, 2 * instruction.1 - dot.1));
            }
            if dot.1 == instruction.1 {
                to_remove.insert(dot.clone());
            }
        }
    }
    (to_remove, to_add)
}
fn print_letters(dots: HashSet<(u32, u32)>) {
    for i in 0..((&dots).into_iter().map(|(_a, b)| b).max().unwrap() + 1) {
        for j in 0..((&dots).into_iter().map(|(a, _b)| a).max().unwrap() + 1) {
            if dots.contains(&(j, i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn print_solution(mut dots: HashSet<(u32, u32)>, instructions: Vec<(char, u32)>) {
    let mut first_instruction = true;
    for instruction in instructions {
        let (to_remove, to_add) = fold(&dots, instruction);

        for dot in to_remove {
            dots.remove(&dot);
        }
        for dot in to_add {
            dots.insert(dot);
        }
        if first_instruction {
            println!("{}", dots.len());
            first_instruction = false;
        }
    }

    print_letters(dots);
}

fn main() {
    let (dots, instructions) = read_lines_to_map("./input");
    print_solution(dots, instructions);
}
