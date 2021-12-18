fn solve(target_x: (i64, i64), target_y: (i64, i64), is_part2: bool) -> i64 {
    let mut max_heights: Vec<i64> = Vec::new();

    for x in 0..(target_x.1 + 1) {
        for y in target_y.0..(-2 * target_y.0) {
            let mut cy = y;
            let mut cx = x;
            let mut posy = 0;
            let mut posx = 0;
            let mut maxy = 0;
            for _steps in 0..(-2 * target_y.0) {
                posx += cx;
                posy += cy;
                if cy == 0 {
                    maxy = posy;
                }
                if target_x.0 <= posx
                    && posx <= target_x.1
                    && target_y.0 <= posy
                    && posy <= target_y.1
                {
                    max_heights.push(maxy);
                    break;
                }
                if cx != 0 {
                    cx -= 1;
                }
                cy -= 1;
            }
        }
    }

    if is_part2 {
        max_heights.len() as i64
    } else {
        max_heights.into_iter().max().unwrap()
    }
}

fn main() {
    let target_x = (32, 65);
    let target_y = (-225, -177);
    println!("{}", solve(target_x, target_y, false));
    println!("{}", solve(target_x, target_y, true));
}
