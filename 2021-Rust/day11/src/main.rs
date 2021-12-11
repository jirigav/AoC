use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines_to_map(file_path: &str) -> HashMap<(usize, usize), u32> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut map: HashMap<(usize, usize), u32> = HashMap::new();
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    for i in 0..lines.len() {
        let chars: Vec<char> = lines[i].chars().collect();
        for j in 0..chars.len() {
            map.insert((i, j), chars[j].to_digit(10).unwrap());
        }
    }
    map
}

fn get_neighbours(i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    for a in [-1, 0, 1] {
        for b in [-1, 0, 1] {
            if a == 0 && b == 0 {
                continue;
            }
            if ((i as i32) + a) >= 0
                && ((i as i32) + a) < 10
                && ((j as i32) + b) >= 0
                && ((j as i32) + b) < 10
            {
                neighbours.push((((i as i32) + a) as usize, ((j as i32) + b) as usize))
            }
        }
    }
    neighbours
}

fn increment_all(map: &mut HashMap<(usize, usize), u32>) -> bool {
    let mut found = false;
    for (_, v) in map.iter_mut() {
        *v = *v + 1;
        found = found || (*v + 1) > 9;
    }
    found
}

fn increase_flashed_neighbours(
    map: &mut HashMap<(usize, usize), u32>,
    flashed: &mut HashSet<(usize, usize)>,
) -> bool {
    let mut found = false;

    for i in 0..10 {
        for j in 0..10 {
            if *map.get(&(i, j)).unwrap() > 9 {
                for (a, b) in get_neighbours(i, j).into_iter() {
                    if map.contains_key(&(a, b)) && !flashed.contains(&(a, b)) {
                        map.insert((a, b), map.get(&(a, b)).unwrap() + 1);
                    }
                }
                found = true;
                map.insert((i, j), 0);
                flashed.insert((i, j));
            }
        }
    }
    found
}

fn solve(mut map: HashMap<(usize, usize), u32>, is_part2: bool) -> usize {
    let mut counter = 0;
    let mut r = 1;
    loop {
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        let mut found = increment_all(&mut map);

        while found {
            found = increase_flashed_neighbours(&mut map, &mut flashed);
        }

        counter += flashed.len();
        if r == 100 && !is_part2 {
            break;
        }

        if flashed.len() == 100 {
            return r;
        }
        r += 1
    }
    counter
}

fn main() {
    let map = read_lines_to_map("./input");

    println!("{:?}", solve(map.clone(), false));
    println!("{:?}", solve(map, true));
}
