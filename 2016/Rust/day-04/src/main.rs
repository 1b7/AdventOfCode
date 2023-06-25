use regex::Regex;

fn main() {
    let input = load_input();
    println!("Part 1: {}", process_input(&input));
    
}

fn as_checksum(cs: &[u8; 26]) -> String {
    let mut cs: Vec<_> = cs.iter().enumerate().collect();
    // Ties implicitly broken alphabetically as sort is stable, 
    // and data comes in alphabetic order.
    cs.sort_by(|a, b| b.1.cmp(a.1));
    cs[0..5].iter().map(|&(idx, _)| (idx as u8 + b'a') as char).collect()

}

fn decipher(enciphered: &str, rot: u8) -> String {
    let rotate_char = |c: char| -> char {
        if !c.is_alphabetic() { return c }
        ((((c as u8 - b'a') + rot) % 26) + b'a') as char
    };
    enciphered.chars().map(|c| rotate_char(c)).collect()
}

fn process_input(lines: &str) -> u32 {
    let re = Regex::new(r"(.+)-(\d+)\[(.+)\]").unwrap();
    let mut sum = 0;
    let mut p2_done = false;

    for caps in re.captures_iter(lines) {
        let mut letters = [0u8; 26];
        let enciphered = caps.get(1).unwrap().as_str();

        for c in enciphered.chars() {
            if c == '-' { continue };
            letters[(c as u8 - b'a') as usize] += 1;
        }
        
        let checksum = caps.get(3).unwrap().as_str();
        if as_checksum(&letters) == checksum {
            let id: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
            sum += id;

            if !p2_done && decipher(enciphered, (id % 26) as u8).contains("north") { 
                println!("Part 2: {}", id);
                p2_done = true;
            }
        }
    }
    sum
}


fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}