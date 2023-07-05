use md5;

fn main() {
    let prefix = load_input();
    println!("Part 1: {}", p1(&prefix, 0));
    println!("Part 2: {}", p1(&prefix, 2016));
}

fn hash(pref: &str, suff: usize, stretch: usize, hashes: &mut Vec<md5::Digest>) -> Vec<u8> {
    if suff < hashes.len() {
        return hash_str(&hashes[suff]);
    }
    
    let mut digest = md5::compute(&format!("{}{}", pref, suff));
    for _ in 0..stretch {
        digest = md5::compute(format!("{:02x}", digest));
    }
    
    hashes.push(digest);
    hash_str(&digest)
}

fn p1(prefix: &str, stretch: usize) -> usize {
    let mut hashes = vec![]; // Avoid recalculating stretched hashes.
    let mut suffix = 0;
    let mut key_no = 1;

    loop {
        if let Some(x) = has_triplet(&hash(prefix, suffix, stretch, &mut hashes)) {
            for s in (suffix + 1)..=(suffix + 1000) {
                if contains_quintet_of(&hash(prefix, s, stretch, &mut hashes), x) {
                    if key_no == 64 {
                        return suffix
                    } else {
                        key_no += 1;
                    }
                }
            }
        }
        suffix += 1;
    }
    unreachable!();
}

fn has_triplet(xs: &[u8]) -> Option<u8> {
    for window in xs.windows(3) {
        if window[0] == window[1] && window[1] == window[2] {
            return Some(window[0])
        }
    }
    None
}

fn contains_quintet_of(xs: &[u8], n: u8) -> bool {
    for window in xs.windows(5) {
        if window.iter().all(|&x| x == n) {
            return true
        }
    }
    false
}

fn hash_str(hash: &[u8; 16]) -> Vec<u8> {
    hash.iter().flat_map(|x| [(x >> 4), (x & 15)]).collect()
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}