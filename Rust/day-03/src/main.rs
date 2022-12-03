fn main() {
    let input = include_str!("../../../input/03");
    println!("{}", duplicates(input));
    println!("{}", badges(input));
}

fn cval(c: char) -> usize {
    let mut x: u32 = c.into();
    x -= if c > 'a' { 96 } else { 65 - 27 };
    (x).try_into().unwrap()
}

fn duplicates(s: &str) -> usize {
    let mut ps = 0;
    for line in s.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        for c in left.chars() {
            if right.contains(c) {
                ps += cval(c);
                break
            }
        }
    }
    ps
}

fn badges(s: &str) -> usize {
    let mut ps = 0;
    let ls: Vec<_> = s.lines().collect();
    for i in (0..ls.len()).step_by(3) {
        ps += ls[i].chars().find(|&c| ls[i + 1].contains(c) && ls[i + 2].contains(c))
            .map(|c| cval(c))
            .unwrap_or(0);
    }
    ps
}