fn main() {
    let input = include_str!("../../../input/03");
    println!("{}", duplicates(input));
    println!("{}", badges(input));
}

fn cval(c: char) -> usize {
    let x = c as usize;
    if c > 'a' { x - 96 } else { x - (65 - 27) }
}

fn duplicates(s: &str) -> usize {
    let mut ps = 0;
    for line in s.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        ps += cval( left.chars().find(|&c| right.contains(c)).unwrap() );
    }
    ps
}

fn badges(s: &str) -> usize {
    let mut ps = 0;
    let lines: Vec<_> = s.lines().collect();
    for ls in lines.chunks(3) {
        ps += ls[0].chars().find(|&c| ls[1].contains(c) && ls[2].contains(c))
            .map(|c| cval(c))
            .unwrap_or(0);
    }
    ps
}