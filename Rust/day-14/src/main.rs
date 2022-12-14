use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("../../../input/14");
    let time = |f: &dyn Fn(&str) -> usize| {
        let s = Instant::now();
        (f(input), s.elapsed())
    };

    let (p1, e_p1) = time(&p1);
    let (p2, e_p2) = time(&p2);
    println!("Part 1: {} ({}ms)", p1, e_p1.as_millis());
    println!("Part 2: {} ({}ms)", p2, e_p2.as_millis());
}

fn swap(t: (usize, usize)) -> (usize, usize) { (t.1, t.0) }

fn to_set(input: &str) -> HashSet<(usize, usize)> {
    let mut points = HashSet::new();
    for line in input.lines() {
        let pairs = line.split(" -> ");
        for (left, right) in pairs.clone().zip(pairs.skip(1)) {
            let (a, b) = left.split_once(',').unwrap();
            let (x, y) = right.split_once(',').unwrap();
            let (mut a, mut b, mut x, mut y) = (
                a.parse().unwrap(),
                b.parse().unwrap(),
                x.parse().unwrap(),
                y.parse().unwrap(),
            );

            // Swapping to avoid empty ranges (10..1 is empty in Rust)
            if a > x { (a, x) = swap((a, x)); }
            if b > y { (b, y) = swap((b, y)); }
            (a..=x).for_each(|m| points.extend((b..=y).map(|n| (m, n))));
        }
    }
    points
}

fn p1(input: &str) -> usize { px(input, 1) }
fn p2(input: &str) -> usize { px(input, 2) }
fn px(input: &str, part: u8) -> usize {
    let spawn_point = (500, 0);
    let mut init = to_set(input);

    let floor_y = init.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1 + 2;
    let right_x = init.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0 + 1000;
    let lowest_y = init.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    init.extend((0..=right_x).map(|x| (x, floor_y)));
    let mut points = init.clone();
    let mut has_changed = true;
    
    while has_changed {
        let mut tentative = spawn_point;
        loop {
            let prior = tentative;
            tentative.1 += 1;

            if (part == 1 && tentative.1 > lowest_y) || (part == 2 && points.contains(&spawn_point)) {
                has_changed = false;
                break;
            }

            if points.contains(&tentative) {
                tentative.0 -= 1;
                if points.contains(&tentative) {
                    tentative.0 += 2;
                    if points.contains(&tentative) {
                        points.insert(prior);
                        break;
                    }
                }
            }
        }
    }
    points.difference(&init).count()
}