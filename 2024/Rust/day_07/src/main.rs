use std::collections::HashSet;
use rayon::prelude::*;

fn main() {
    let input = include_str!("../../../input/07");

    let results = input.lines().par_bridge().map(|line| {
        let (target, elems) = line.split_once(':').unwrap();
        let target = target.parse::<usize>().unwrap();
        let elems = elems
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("Cannot parse number from input"))
            .rev()
            .collect::<Vec<_>>();

        (
            if combinations(&elems, false).contains(&target) { target } else { 0 },
            if combinations(&elems, true).contains(&target) { target } else { 0 }
        )
    }).collect::<Vec<_>>();

    let (mut part_one, mut part_two) = (0, 0);
    for (p1, p2) in results {
        part_one += p1;
        part_two += p2;
    }

    println!("Part 1: {part_one}");
    println!("Part 2: {part_two}");
}

fn combinations(xs: &[usize], part_two: bool) -> HashSet<usize> {
    let mut computed = HashSet::new();
    if xs.len() == 1 {
        computed.insert(xs[0]);
        return computed;
    }

    let result = combinations(&xs[1..], part_two);

    for n in result.iter() {
        computed.insert(xs[0] * n);
        computed.insert(xs[0] + n);

        if part_two {
            computed.insert(
                format!("{n}{}", xs[0]).parse::<usize>()
                    .expect("Failed to parse number from concatenation"),
            );
        }
    }

    computed
}
