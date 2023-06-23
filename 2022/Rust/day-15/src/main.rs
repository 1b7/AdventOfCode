use std::collections::{HashSet, HashMap};
use rayon::prelude::*;

fn main() {
    let input = include_str!("../../../input/15");

    let p1 = p1(input, 2000000);
    let p2 = p2(input, 4_000_000);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn manhattan(a: (isize, isize), b: (isize, isize)) -> usize {
    (a.0).abs_diff(b.0) + (a.1).abs_diff(b.1)
}

fn test_is_closer(b: ((isize, isize), (isize, isize)), t: (isize, isize)) -> bool {
    manhattan(b.0, b.1) >= manhattan(b.0, t)
}

fn to_pairs(s: &str) -> ((isize, isize), (isize, isize)) {
    let splits: Vec<isize> = s.split_whitespace()
        .filter(|s| s.starts_with("x=") || s.starts_with("y="))
        .map(|s| s.chars().filter(|&c| c.is_ascii_digit() || c == '-').collect::<String>().parse().unwrap())
        .collect();
    ((splits[0], splits[1]), (splits[2], splits[3]))
}

fn as_radius(pair: ((isize, isize),(isize, isize))) -> ((isize, isize), isize) {
    let (from, to) = pair;
    (from, manhattan(from, to) as isize)
}

fn to_range(pair: ((isize, isize), isize)) -> Vec<((isize, isize), (isize, isize))> {
    let (orig, radius) = pair;
    ((orig.1 - radius)..=(orig.1 + radius)).map(|y| {
        let d = radius - orig.1.abs_diff(y) as isize;
        ((orig.0 - d, y),(orig.0 + d, y))
    }).collect()
}

fn p1(input: &str, target: isize) -> usize {
    let beacons: Vec<_> = input.lines().map(to_pairs).collect();
    let occupied: HashSet<_> = beacons.iter().map(|b| b.1).collect();
    // occupied.extend(beacons.iter().map(|b| b.0)); // - not making a difference to output

    let upper = 10_000_000;
    let lower = -10_000_000;

    let mut count = 0;
    for x in lower..=upper {
        let new = (x, target);
        for &beacon in &beacons {
            if !occupied.contains(&new) && test_is_closer(beacon,  new) {
                count += 1;
                break
            }
        }
    }
    count
}

fn p2(input: &str, target: isize) -> usize {
    let beacons: Vec<_> = input.lines()
        .map(to_pairs)
        .map(as_radius)
        .map(to_range)
        .collect();

    let mut rows: HashMap<isize, Vec<(isize, isize)>> = HashMap::new();
    for b in beacons {
        b.iter().for_each(|p| {
            assert_eq!(p.0.1, p.1.1);
            if rows.contains_key(&p.0.1) {
                rows.get_mut(&p.0.1).unwrap().push((p.0.0, p.1.0));
            } else {
                rows.insert(p.0.1, vec![(p.0.0, p.1.0)]);
            }
        })
    }

    let res = (0..=target).into_par_iter().find_map_any(|y| {
        if let Some(row) = rows.get(&y) {
            for x in 0..=target {
                if !row.iter().any(|&(left, right)| x >= left && x <= right) {
                    return Some((x, y));
                }
            }
        } else { eprintln!("Nothing in row {}", y); }
        None
    });

    println!("{:?}", res);
    
    (res.unwrap().0 * 4000000 + res.unwrap().1) as usize
}
