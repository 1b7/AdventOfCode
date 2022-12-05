fn main() {
    let input = include_str!("../../../input/05");
    println!("Part 1: {}", arrange(input, &shift_seq));
    println!("Part 2: {}", arrange(input, &shift_batch));
}

fn to_crates(crates: &str) -> Vec<Vec<char>> {
    let mut crate_stacks = vec![vec![]; 9];
    for row in crates.lines() {
        for (i, c) in row.char_indices().skip(1).step_by(4) {
            if c > '9' { crate_stacks[(i - 1) / 4].push(c); }
        }
    }
    for s in &mut crate_stacks { s.reverse(); }
    crate_stacks
}

fn parse_instruction(instruction: &str) -> (usize, usize, usize) {
    let split: Vec<usize> = instruction.split_whitespace()
        .filter_map(|n| match n.parse::<usize>() {
            Ok(n) => Some(n),
            _ => None
        })
        .collect();
    (split[0], split[1], split[2])
}

fn shift_seq(stacks: &mut Vec<Vec<char>>, qty: usize, from: usize, to: usize) {
    for _ in 0..qty {
        let p = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(p);
    }
}

fn shift_batch(stacks: &mut Vec<Vec<char>>, qty: usize, from: usize, to: usize) {
    let mut tmp = vec![];
    for _ in 0..qty { tmp.push(stacks[from - 1].pop().unwrap()); }
    for _ in 0..qty { stacks[to - 1].push(tmp.pop().unwrap()); }
}

fn arrange(s: &str, f: &dyn Fn (&mut Vec<Vec<char>>, usize, usize, usize) -> ()) -> String {
    let (crates, instructions) = s.split_once("\r\n\r\n").unwrap();
    let mut crate_stacks = to_crates(crates);

    for instr in instructions.lines() {
        let (qty, from, to) = parse_instruction(instr);
        f(&mut crate_stacks, qty, from, to);
    }
    crate_stacks.iter().map(|s| s.last().unwrap()).collect()
} 