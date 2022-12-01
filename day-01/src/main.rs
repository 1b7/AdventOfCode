fn main() {
    let (p1, p2) = largest();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn largest() -> (i32, i32) {
    let s = include_str!("../../input/01");
    let mut maxes = vec![];
    let mut count = 0;

    for line in s.lines() {
        if line.is_empty() {
            maxes.push(count);
            count = 0;
        } else {
            count += line.parse::<i32>().unwrap();
        }
    }
    maxes.sort();
    maxes.reverse();
    (maxes[0], maxes[0..3].iter().sum())
}