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

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert(rope[0]);

    for line in input.lines() {
        let (dir, n) = line.split_once(' ').unwrap();
        let n: usize = n.parse().unwrap();

        for _ in 0..n {
            // Manually iterating over vec (windows has no mut equivalent)
            for i in 0..(rope.len() - 1) {
                let mut head = rope[i];
                let mut tail = rope[i + 1];

                // Only apply instruction to actual head of rope:
                if i == 0 {
                    head = match dir {
                        "R" => (head.0 + 1, head.1),
                        "L" => (head.0 - 1, head.1),
                        "U" => (head.0, head.1 + 1),
                        "D" => (head.0, head.1 - 1),
                        _ => panic!()
                    };
                };
    
                let dif_y = (head.1 - tail.1).abs();
                let dif_x = (head.0 - tail.0).abs();
    
                if dif_y + dif_x > 2 {
                    tail = if (head.0 - tail.0) > 1 {
                        if head.1 > tail.1 {
                            (tail.0 + 1, tail.1 + 1)
                        } else {
                            (tail.0 + 1, tail.1 - 1)
                        }
                    } else if (head.1 - tail.1) > 1 {
                        if head.0 > tail.0 {
                            (tail.0 + 1, tail.1 + 1)
                        } else {
                            (tail.0 - 1, tail.1 + 1)
                        }
                    } else if (tail.0 - head.0) > 1 {
                        if head.1 > tail.1 {
                            (tail.0 - 1, tail.1 + 1)
                        } else {
                            (tail.0 - 1, tail.1 - 1)
                        }
                    } else {
                        if head.0 > tail.0 {
                            (tail.0 + 1, tail.1 - 1)
                        } else {
                            (tail.0 - 1, tail.1 - 1)
                        }
                    }
                } else if dif_x > 1 {
                    if head.0 > tail.0 {
                        tail = (head.0 - 1, tail.1)
                    } else {
                        tail = (head.0 + 1, tail.1)
                    }
                } else if dif_y > 1 {
                    if head.1 > tail.1 {
                        tail = (tail.0, head.1 - 1)
                    } else {
                        tail = (tail.0, head.1 + 1)
                    }
                }

                rope[i] = head;
                rope[i + 1] = tail;
            }
            visited_tail.insert(rope[1]);
            visited.insert(rope[nk - 1]);
        }
    }
    (visited_tail.len(), visited.len())
}