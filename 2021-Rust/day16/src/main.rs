use std::fs::File;
use std::io::{BufRead, BufReader};
use to_binary::BinaryString;

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    type_id: u8,
    value: i64,
    subpackets: Vec<Packet>,
}

impl Packet {
    pub fn new(mut bin_str: &str) -> (Self, &str) {
        let version = u8::from_str_radix(&bin_str[0..3], 2).unwrap();
        let type_id = u8::from_str_radix(&bin_str[3..6], 2).unwrap();
        bin_str = &bin_str[6..];

        let mut bin_value: String = String::new();
        let mut value = 0;
        let mut subpackets = Vec::new();

        if type_id == 4 {
            loop {
                bin_value += &bin_str[1..5];
                if &bin_str[0..1] == "0" {
                    bin_str = &bin_str[5..];
                    break;
                }
                bin_str = &bin_str[5..];
            }
            value = i64::from_str_radix(&bin_value, 2).unwrap();
        } else {
            if &bin_str[0..1] == "0" {
                let len = 15;
                let subpackets_len = i64::from_str_radix(&bin_str[1..(len + 1)], 2).unwrap();
                bin_str = &bin_str[(len + 1)..];
                let mut subpackets_bin = &bin_str[0..(subpackets_len as usize)];
                bin_str = &bin_str[(subpackets_len as usize)..];
                while subpackets_bin.contains("1") {
                    let a = Packet::new(subpackets_bin);
                    let p = a.0;
                    subpackets_bin = a.1;
                    subpackets.push(p);
                }
            } else {
                let len = 11;
                let subpackets_count = i64::from_str_radix(&bin_str[1..(len + 1)], 2).unwrap();
                bin_str = &bin_str[(len + 1)..];
                let mut count = 0;
                while subpackets_count > count {
                    count += 1;
                    let a = Packet::new(bin_str);
                    let p = a.0;
                    bin_str = a.1;
                    subpackets.push(p);
                }
            }
        }

        (
            Packet {
                version,
                type_id,
                value,
                subpackets,
            },
            bin_str,
        )
    }
}

fn read_lines_strings(file_path: &str) -> String {
    let file = File::open(file_path).expect("file wasn't found.");
    let mut reader = BufReader::new(file);
    let mut line: String = String::new();
    reader.read_line(&mut line);
    line.pop();
    line
}

fn dfs_versions(p: Packet) -> i64 {
    let mut v = p.version as i64;
    for subp in p.subpackets.into_iter() {
        v += dfs_versions(subp);
    }
    v
}

fn part1(input: String) -> i64 {
    let input = BinaryString::from_hex(input).unwrap().to_string();
    let (p, _) = Packet::new(&input);
    dfs_versions(p)
}

fn apply_operator(p: &Packet, o: fn(i64, i64) -> i64) -> i64 {
    let s = dfs_expressions(&p.subpackets[0]);
    (&p.subpackets)
        .into_iter()
        .skip(1)
        .fold(s, |a, b| o(a, dfs_expressions(&b)))
}

fn gt(a: i64, b: i64) -> i64 {
    if a > b {
        1
    } else {
        0
    }
}

fn lt(a: i64, b: i64) -> i64 {
    if a < b {
        1
    } else {
        0
    }
}

fn eq(a: i64, b: i64) -> i64 {
    if a == b {
        1
    } else {
        0
    }
}

fn dfs_expressions(p: &Packet) -> i64 {
    match p.type_id {
        0 => apply_operator(p, i64::wrapping_add),
        1 => apply_operator(p, i64::wrapping_mul),
        2 => apply_operator(p, i64::min),
        3 => apply_operator(p, i64::max),
        4 => p.value,
        5 => apply_operator(p, gt),
        6 => apply_operator(p, lt),
        7 => apply_operator(p, eq),
        _ => 0,
    }
}

fn part2(input: String) -> i64 {
    let input = BinaryString::from_hex(input).unwrap().to_string();
    let (p, _) = Packet::new(&input);
    dfs_expressions(&p)
}

fn main() {
    let input = read_lines_strings("./input");
    println!("{}", part1(input.clone()));
    println!("{}", part2(input));
}
