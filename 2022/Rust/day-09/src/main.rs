use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../input/09");
    let (p1, p2) = simulate(input, 10);
    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
}

fn simulate(input: &str, nk: usize) -> (usize, usize) {
    let mut rope = vec![(0, 0); nk];

    let mut visited_tail: HashSet<(isize, isize)> = HashSet::new();
    visited_tail.insert(rope[0]);

    let mut visited_first: HashSet<(isize, isize)> = HashSet::new();
    visited_first.insert(rope[0]);

    for line in input.lines() {
        let (dir, n) = line.split_once(' ').unwrap();
        let n: usize = n.parse().unwrap();

        for _ in 0..n {
            for i in 0..(rope.len() - 1) {
                let mut head = rope[i];
                let mut tail = rope[i + 1];
                
                if i == 0 { // Only apply instruction to actual head of rope:
                    head = match dir {
                        "R" => (head.0 + 1, head.1),
                        "L" => (head.0 - 1, head.1),
                        "U" => (head.0, head.1 + 1),
                        "D" => (head.0, head.1 - 1),
                        _ => panic!()
                    };
                };

                if (head.1 - tail.1).abs() > 1 || (head.0 - tail.0).abs() > 1 {
                    tail.0 += (head.0 - tail.0).signum();
                    tail.1 += (head.1 - tail.1).signum();
                }

                rope[i] = head;
                rope[i + 1] = tail;
            }
            visited_first.insert(rope[1]);
            visited_tail.insert(rope[nk - 1]);
        }
    }
    (visited_first.len(), visited_tail.len())
}