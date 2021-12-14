use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(file_path: &str) -> (String, HashMap<String, (String, String)>) {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut insertions: HashMap<String, (String, String)> = HashMap::new();
    let mut lines = reader.lines().map(|line| line.unwrap());
    let template = lines.next().unwrap();
    lines.next();

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        insertions.insert(
            format!("{}{}", chars[0], chars[1]),
            (
                format!("{}{}", chars[0], chars[6]),
                format!("{}{}", chars[6], chars[1]),
            ),
        );
    }
    (template, insertions)
}

fn add_to_count<T: Eq + std::hash::Hash + Clone>(counter: &mut HashMap<T, i64>, str: T, v: i64) {
    if counter.contains_key(&str) {
        counter.insert(str.clone(), counter.get(&str).unwrap() + v);
    } else {
        counter.insert(str, v);
    }
}

fn solve(template: &String, insertions: &HashMap<String, (String, String)>, is_part2: bool) -> i64 {
    let mut count_pairs: HashMap<String, i64> = HashMap::new();
    let template_chars: Vec<char> = template.chars().collect();
    for i in 0..(template.len() - 1) {
        let mut str = template_chars[i].to_string();
        str.push(template_chars[i + 1]);
        add_to_count(&mut count_pairs, str, 1);
    }

    for i in 0..40 {
        let mut new_count: HashMap<String, i64> = HashMap::new();

        for (pair, count) in count_pairs.clone().into_iter() {
            let new_pairs = insertions.get(&pair).unwrap();
            add_to_count(&mut new_count, new_pairs.0.clone(), count);
            add_to_count(&mut new_count, new_pairs.1.clone(), count);
        }
        count_pairs = new_count;

        if !is_part2 && i == 9 {
            break;
        }
    }

    let mut count_letters: HashMap<char, i64> = HashMap::new();
    count_letters.insert(template.chars().next().unwrap(), 1);
    count_letters.insert(template.chars().last().unwrap(), 1);

    for (pair, count) in count_pairs.into_iter() {
        for l in pair.chars() {
            add_to_count(&mut count_letters, l, count);
        }
    }

    let mut v: Vec<i64> = count_letters.values().map(|x| (*x) / 2).collect();
    v.sort();
    v[v.len() - 1] - v[0]
}

fn main() {
    let (template, insertions) = parse("./input");
    println!("{:?}", solve(&template, &insertions, false));
    println!("{:?}", solve(&template, &insertions, true));
}
