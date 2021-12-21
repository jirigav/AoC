fn turn1(pos: &mut i64, score: &mut i64, dice: &mut i64) {
    let mut rolled = 0;
    for _ in 0..3 {
        *dice = (*dice + 1) % 100;
        if dice == &0 {
            rolled += 100;
        } else {
            rolled += *dice;
        }
    }

    *pos = (*pos + rolled) % 10;
    if pos == &0 {
        *score += 10;
    } else {
        *score += *pos
    }
}

fn part1(mut pos1: i64, mut pos2: i64) -> i64 {
    let mut score1 = 0;
    let mut score2 = 0;
    let mut dice = 0;
    let mut round = 0;
    while score1 < 1000 && score2 < 1000 {
        if round % 2 == 0 {
            turn1(&mut pos1, &mut score1, &mut dice);
        } else {
            turn1(&mut pos2, &mut score2, &mut dice);
        }
        round += 1;
    }
    if score1 > score2 {
        score2 * round * 3
    } else {
        score1 * round * 3
    }
}

fn turn2(pos: i64, score: i64, pos2: i64, score2: i64, player1: bool) -> (i64, i64) {
    let dice_results = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    let mut wins1 = 0;
    let mut wins2 = 0;
    for (result, count) in dice_results {
        let p = (pos + result) % 10;
        let mut sc = 0;
        if p == 0 {
            sc += score + 10;
        } else {
            sc = score + p;
        }
        if sc > 20 {
            if player1 {
                wins1 += count;
            } else {
                wins2 += count;
            }
        } else {
            let scores: (i64, i64);
            if player1 {
                scores = dfs_quantum_turns(p, pos2, sc, score2, false);
            } else {
                scores = dfs_quantum_turns(pos2, p, score2, sc, true);
            }
            wins1 += count * scores.0;
            wins2 += count * scores.1;
        }
    }
    (wins1, wins2)
}

fn dfs_quantum_turns(pos1: i64, pos2: i64, score1: i64, score2: i64, player1: bool) -> (i64, i64) {
    let wins1;
    let wins2;
    if player1 {
        let wins = turn2(pos1, score1, pos2, score2, player1);
        wins1 = wins.0;
        wins2 = wins.1;
    } else {
        let wins = turn2(pos2, score2, pos1, score1, player1);
        wins1 = wins.0;
        wins2 = wins.1;
    }
    (wins1, wins2)
}

fn part2(pos1: i64, pos2: i64) -> i64 {
    let (wins1, wins2) = dfs_quantum_turns(pos1, pos2, 0, 0, true);
    wins1.max(wins2)
}

fn main() {
    let pos1 = 2;
    let pos2 = 10;

    println!("{}", part1(pos1, pos2));
    println!("{:?}", part2(pos1, pos2));
}
