use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines_to_map(file_path: &str) -> (HashMap<String, HashSet<String>>, HashMap<String, bool>) {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut is_large: HashMap<String, bool> = HashMap::new();
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    for line in lines {
        let split: Vec<&str> = line.split("-").collect();
        for i in [0, 1] {
            if split[i].chars().next().unwrap().is_uppercase() {
                is_large.insert(split[i].to_string(), true);
            } else {
                is_large.insert(split[i].to_string(), false);
            }
            if map.contains_key(split[i]) {
                map.get_mut(split[i])
                    .unwrap()
                    .insert(split[(i + 1) % 2].to_string());
            } else {
                let mut neighbours = HashSet::new();
                neighbours.insert(split[(i + 1) % 2].to_string());
                map.insert(split[i].to_string(), neighbours);
            }
        }
    }
    (map, is_large)
}

fn dfs_count(
    map: &HashMap<String, HashSet<String>>,
    is_large: &HashMap<String, bool>,
    vertex: String,
    mut visited: HashSet<String>,
    revisited: bool,
    is_part2: bool,
) -> i64 {
    if vertex == "end".to_string() {
        return 1;
    }
    if !is_large.get(&vertex).unwrap() {
        visited.insert(vertex.clone());
    }
    let mut count: i64 = 0;
    for n in map.get(&vertex).unwrap().into_iter() {
        if !visited.contains(n) {
            count += dfs_count(
                map,
                is_large,
                n.clone(),
                visited.clone(),
                revisited,
                is_part2,
            )
        } else if is_part2 && !revisited && *n != "start".to_string() {
            count += dfs_count(map, is_large, n.clone(), visited.clone(), true, is_part2)
        }
    }

    count
}

fn solve(
    map: &HashMap<String, HashSet<String>>,
    is_large: &HashMap<String, bool>,
    is_part2: bool,
) -> i64 {
    let visited: HashSet<String> = HashSet::new();
    dfs_count(
        &map,
        is_large,
        "start".to_string(),
        visited,
        false,
        is_part2,
    )
}

fn main() {
    let (map, is_large) = read_lines_to_map("./input");

    println!("{:?}", solve(&map, &is_large, false));
    println!("{:?}", solve(&map, &is_large, true));
}
