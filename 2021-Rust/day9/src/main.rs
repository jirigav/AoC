use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines_nums(file_path: &str) -> Vec<Vec<u32>> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .into_iter()
                .map(|x| x.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn get_neighbours(i: i32, j: i32, input_len: i32) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    for d in [-1, 1] {
        if i + d >= 0 && i + d < input_len {
            neighbours.push(((i + d) as usize, j as usize));
        }

        if j + d >= 0 && j + d < input_len {
            neighbours.push((i as usize, (j + d) as usize));
        }
    }
    neighbours
}

fn is_lowpoint(i: i32, j: i32, input: &Vec<Vec<u32>>) -> bool {
    for (a, b) in get_neighbours(i, j, input.len() as i32) {
        if input[a][b] <= input[i as usize][j as usize] {
            return false;
        }
    }
    true
}

fn get_lowpoints(input: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut lowpoints: Vec<(usize, usize)> = Vec::new();

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if is_lowpoint(i as i32, j as i32, input) {
                lowpoints.push((i, j));
            }
        }
    }
    lowpoints
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    get_lowpoints(input)
        .into_iter()
        .map(|(i, j)| input[i][j] + 1)
        .sum()
}

fn dfs_count(map: &mut Vec<Vec<(u32, bool)>>, i: i32, j: i32) -> u32 {
    let mut count = 0;
    for (a, b) in get_neighbours(i, j, map.len() as i32) {
        if map[a][b].0 != 9 && !map[a][b].1 {
            map[a][b].1 = true;
            count += 1;
            count += dfs_count(map, a as i32, b as i32);
        }
    }
    count
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let lowpoints = get_lowpoints(input);

    let mut basin_sizes: Vec<u32> = Vec::new();

    let mut map: Vec<Vec<(u32, bool)>> = input
        .into_iter()
        .map(|x| x.into_iter().map(|x| (*x, false)).collect())
        .collect();

    for (i, j) in lowpoints {
        basin_sizes.push(dfs_count(&mut map, i as i32, j as i32));
    }

    basin_sizes.sort();
    basin_sizes[basin_sizes.len() - 1]
        * basin_sizes[basin_sizes.len() - 2]
        * basin_sizes[basin_sizes.len() - 3]
}

fn main() {
    let input = read_lines_nums("./input");
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}
