use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../input/04");
    let (p1, p2) = intersections(input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn intersections(s: &str) -> (usize, usize) {
    let split_to_range = |s: &str| -> HashSet<usize> {
        let (l, r) = s.split_once('-').unwrap();
        (l.parse().unwrap()..=r.parse().unwrap()).collect()
    };

    let mut supers = 0;
    let mut overlaps = 0;
    for line in s.lines() {
        let (left, right) = line.split_once(',').unwrap();
        let (l, r) = (split_to_range(left), split_to_range(right));
        if l.is_subset(&r) || r.is_subset(&l) { supers += 1; }
        if !l.is_disjoint(&r) { overlaps += 1; }
    }
    (supers, overlaps)
}
