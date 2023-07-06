const SIZE: i8 = 4;
fn main() {
    let (shortest, longest) = bfs(&load_input());
    println!("Part 1: {}", shortest);
    println!("Part 2: {}", longest.len());
}

fn bfs(passcode: &str) -> (String, String) {
    let mut frontier: Vec<String> = vec!["".to_owned()];
    let mut next_frontier = vec![];
    let mut shortest = None;
    let mut longest = String::new();

    while frontier.len() > 0 {
        for next in &frontier {
            let point = movestr_to_point(next);
            if point == (3, 3) {
                if shortest.is_none() {
                    shortest = Some(next.to_owned());
                }
                longest = next.to_owned();
                continue;
            }
            let next_moves = valid_moves(passcode, next, point);
            next_frontier.extend(next_moves);
        }
        frontier = next_frontier.clone();
        next_frontier.clear();
    }
    (shortest.unwrap_or_default(), longest)
}

fn valid_moves(passcode: &str, from: &str, point: (i8, i8)) -> Vec<String> {
    let append = |c: char| {
        let mut new = from.to_owned();
        new.push(c);
        new
    };

    let mut combined = passcode.to_owned();
    combined.push_str(from);
    let open_doors = &hash_str(&md5::compute(&combined))[0..4];

    let (sx, sy) = point;
    let mut moves = vec![];
    if open_doors[0] > 10 && sy > 0 { moves.push(append('U')) }
    if open_doors[1] > 10 && sy < (SIZE - 1) { moves.push(append('D')) }
    if open_doors[2] > 10 && sx > 0 { moves.push(append('L')) }
    if open_doors[3] > 10 && sx < (SIZE - 1) { moves.push(append('R')) }
    moves
}

fn movestr_to_point(s: &str) -> (i8, i8) {
    let (mut x, mut y) = (0, 0);
    for ch in s.chars() {
        match ch {
            'U' => y -= 1,
            'D' => y += 1,
            'L' => x -= 1,
            'R' => x += 1,
             _  => continue
        };
    }
    (x, y)
}
fn hash_str(hash: &[u8; 16]) -> Vec<u8> {
    hash.iter().flat_map(|x| [(x >> 4), (x & 15)]).collect()
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}