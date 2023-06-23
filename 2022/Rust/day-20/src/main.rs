use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../../input/20");
    let mut nums = as_nums(input);
    let p1 = p1(&mut nums.clone());
    let p2 = p2(&mut nums, 811589153);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn shift(i: usize, v: &mut VecDeque<(usize, isize)>) {
    let (loc, &(_, n)) = v.iter()
        .enumerate()
        .find(|(_, m)| m.0 == i).unwrap();
    if n == 0 { return; }

    v.rotate_left(loc);
    v.pop_front();
    let rot = n.rem_euclid(v.len() as isize) as usize;
    v.rotate_left(rot);
    v.push_front((i, n));
}

fn as_nums(s: &str) -> VecDeque<(usize, isize)> {
    s.lines().enumerate()
        .map(|(i, line)| (i, line.parse::<isize>().unwrap()))
        .collect()
}

fn sum_output(nums: &VecDeque<(usize, isize)>) -> isize {
    let (zero_i, _) = nums.iter().enumerate().find(|(_, n)| n.1 == 0).unwrap();
    let ns = [1000, 2000, 3000].map(|x| nums[(x + zero_i) % (nums.len())].1);
    ns.iter().sum()
}

fn p1(nums: &mut VecDeque<(usize, isize)>) -> isize {
    for i in 0..nums.len() { shift(i, nums); }
    sum_output(nums)
}

fn p2(nums: &mut VecDeque<(usize, isize)>, key: usize) -> isize {
    nums.iter_mut().for_each(|(_, n)| *n *= key as isize );
    (0..10).for_each(|_| for i in 0..nums.len() { shift(i, nums); });
    sum_output(nums)
}