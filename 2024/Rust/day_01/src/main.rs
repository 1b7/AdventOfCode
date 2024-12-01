use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

fn main() {
    let (mut left_list, mut right_list) = (BinaryHeap::new(), BinaryHeap::new());
    let mut counts = HashMap::new();

    include_str!("../../../input/01")
        .lines()
        .for_each(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.trim().parse().expect("Error parsing integer"))
                .collect();
            
            left_list.push(Reverse(nums[0]));
            right_list.push(Reverse(nums[1]));
            *counts.entry(nums[1]).or_insert(0) += 1;
        });
    
    let left_list: Vec<_> = left_list.into_sorted_vec();
    let right_list: Vec<_> = right_list.into_sorted_vec();

    let mut p1 = 0;
    let mut p2 = 0;

    for (a, b) in left_list.iter().zip(right_list.iter()) {
        p1 += (b.0 - a.0).abs();
        p2 += *counts.entry(a.0).or_default() * a.0;
    }

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
