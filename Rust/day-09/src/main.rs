use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../input/09");
    println!("Part 1: {:?}", p1(input));
    println!("Part 2: {:?}", p2(input));
}

fn p1(input: &str) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert(tail);

    for line in input.lines() {
        let (dir, n) = line.split_once(' ').unwrap();
        let n: usize = n.parse().unwrap();
        // println!("====== Line: ({}, {}) ======", dir, n);
        for _ in 0..n {
            // println!("PRE - Head: {:?} Tail: {:?}", head, tail);
            head = match dir {
                "R" => (head.0 + 1, head.1),
                "L" => (head.0 - 1, head.1),
                "U" => (head.0, head.1 + 1),
                "D" => (head.0, head.1 - 1),
                _ => panic!()
            };


            let is_adjacent = (tail.0, tail.1) == (head.0, head.1)
                || (tail.0, tail.1) == (head.0 - 1, head.1)
                || (tail.0, tail.1) == (head.0 + 1, head.1)
                || (tail.0, tail.1) == (head.0,     head.1 + 1)
                || (tail.0, tail.1) == (head.0,     head.1 - 1)
                || (tail.0, tail.1) == (head.0 + 1, head.1 + 1)
                || (tail.0, tail.1) == (head.0 - 1, head.1 - 1)
                || (tail.0, tail.1) == (head.0 - 1, head.1 + 1)
                || (tail.0, tail.1) == (head.0 + 1, head.1 - 1);
            
            let dif_height: isize = head.1 - tail.1;
            let dif_width: isize = head.0 - tail.0;

            if ((dif_height.abs() > 1 && dif_width.abs() > 0) || (dif_height.abs() > 0 && dif_width.abs() > 1 )) && !is_adjacent {
                // println!("DIAG CHECK");
                tail = if (head.0 - tail.0) > 1 {
                    // println!("DIAG A:");
                    // (head.0 - 1, tail.1)
                    if head.1 > tail.1 {
                        (tail.0 + 1, tail.1 + 1)
                    } else {
                        (tail.0 + 1, tail.1 - 1)
                    }
                } else if (head.1 - tail.1) > 1 {
                    // println!("DIAG B:");
                    // (tail.0, head.1 - 1)
                    if head.0 > tail.0 {
                        (tail.0 + 1, tail.1 + 1)
                    } else {
                        (tail.0 - 1, tail.1 + 1)
                    }
                } else if (tail.0 - head.0) > 1 {
                    // println!("DIAG C:");
                    // (tail.0 + 1, head.1)
                    if head.1 > tail.1 {
                        (tail.0 - 1, tail.1 + 1)
                    } else {
                        (tail.0 - 1, tail.1 - 1)
                    }
                } else {
                    // println!("DIAG D:");
                    // (tail.0, head.1 + 1)
                    if head.0 > tail.0 {
                        (tail.0 + 1, tail.1 - 1)
                    } else {
                        (tail.0 - 1, tail.1 - 1)
                    }
                }
            } else if dif_width.abs() > 1 {
                // println!("MOVE H");
                if head.0 > tail.0 {
                    tail = (head.0 - 1, tail.1)
                } else {
                    tail = (head.0 + 1, tail.1)
                }
            } else if dif_height.abs() > 1 {
                // println!("MOVE V");
                if head.1 > tail.1 {
                    tail = (tail.0, head.1 - 1)
                } else {
                    tail = (tail.0, head.1 + 1)
                }
            }
            

            // println!("POST - Head: {:?} Tail: {:?} {}", head, tail, is_adjacent);
            // if !is_adjacent { panic!("Not Adjacent") }
            
            visited.insert(tail);
        }

    }
    // println!("{:?}", visited);
    visited.len()
}


fn p2(input: &str) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut knots = vec![(0, 0); 10];
    visited.insert(knots[9]);

    for line in input.lines() {
        let (dir, n) = line.split_once(' ').unwrap();
        let n: usize = n.parse().unwrap();
        // println!("====== Line: ({}, {}) ======", dir, n);
        for _ in 0..n {
            // println!("{:?}", knots);
            for i in 0..(knots.len() - 1) {
                let mut head = knots[i];
                let mut tail = knots[i + 1];

                // Only move 'real' head:
                if i == 0 {
                    head = match dir {
                        "R" => (head.0 + 1, head.1),
                        "L" => (head.0 - 1, head.1),
                        "U" => (head.0, head.1 + 1),
                        "D" => (head.0, head.1 - 1),
                        _ => panic!()
                    };
                };
    
    
                let is_adjacent = (tail.0, tail.1) == (head.0, head.1)
                    || (tail.0, tail.1) == (head.0 - 1, head.1)
                    || (tail.0, tail.1) == (head.0 + 1, head.1)
                    || (tail.0, tail.1) == (head.0,     head.1 + 1)
                    || (tail.0, tail.1) == (head.0,     head.1 - 1)
                    || (tail.0, tail.1) == (head.0 + 1, head.1 + 1)
                    || (tail.0, tail.1) == (head.0 - 1, head.1 - 1)
                    || (tail.0, tail.1) == (head.0 - 1, head.1 + 1)
                    || (tail.0, tail.1) == (head.0 + 1, head.1 - 1);
                
                let dif_height: isize = head.1 - tail.1;
                let dif_width: isize = head.0 - tail.0;
    
                if ((dif_height.abs() > 1 && dif_width.abs() > 0) || (dif_height.abs() > 0 && dif_width.abs() > 1 )) && !is_adjacent {
                    // println!("DIAG CHECK");
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
                } else if dif_width.abs() > 1 {
                    if head.0 > tail.0 {
                        tail = (head.0 - 1, tail.1)
                    } else {
                        tail = (head.0 + 1, tail.1)
                    }
                } else if dif_height.abs() > 1 {
                    if head.1 > tail.1 {
                        tail = (tail.0, head.1 - 1)
                    } else {
                        tail = (tail.0, head.1 + 1)
                    }
                }
                knots[i] = head;
                knots[i + 1] = tail;
                visited.insert(knots[9]);
            }
        }
    }
    visited.len()
}