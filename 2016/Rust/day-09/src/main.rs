fn main() {
    let s: Vec<_> = load_input().chars().collect();
    let (non_rec, rec) = decompress(&s);
    println!("Part 1: {}\nPart 2: {}", non_rec, rec);
}

fn decompress(s: &[char]) -> (usize, usize) {
    let mut i = 0;
    let mut count = 0;
    let mut rec_count = 0;

    while i < s.len() {
        match &s[i] {
            '(' => {
                let mut j = i + 1;
                while s[j] != ')' { j += 1; }
                
                let marker: String = (&s[(i + 1)..j]).iter().collect();
                let (dist, repeat) = marker.split_once('x').unwrap();
                let (dist, repeat) = (dist.parse::<usize>().unwrap(), repeat.parse::<usize>().unwrap());
                
                // Recursively expand nested markers:
                let (_, seg_length) = decompress(&s[(j + 1)..=(j + dist)]);

                rec_count += seg_length * repeat;
                count += dist * repeat;
                i = j + dist + 1;
            },
            _ => {
                rec_count += 1;
                count += 1;
                i += 1;
            }
        }
    }
    (count, rec_count)
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}