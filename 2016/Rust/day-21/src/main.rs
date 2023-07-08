fn main() {
    let ops = load_input().lines().map(as_op).collect::<Vec<_>>();
    let mut pwd = "abcdefgh".chars().collect();
    scramble(&mut pwd, &ops);
    println!("Part 1: {:?}", String::from_iter(pwd.iter()));
    
    let mut scrambled = "fbgdceah".chars().collect();
    unscramble(&mut scrambled, &ops);
    println!("Part 2: {:?}", String::from_iter(scrambled.iter()));

}

fn scramble(pwd: &mut Vec<char>, ops: &[Op]) {
    for op in ops {
        match op {
            Op::SwapPosition(x, y) => swap_pos(*x, *y, pwd),
            Op::SwapLetter(x, y) => swap_letter(*x, *y, pwd),
            Op::RotateLeft(n) => rot_left(*n, pwd),
            Op::RotateRight(n) => rot_right(*n, pwd),
            Op::RotateLetter(c) => rot_letter(*c, pwd),
            Op::Reverse(l, r) => reverse(*l, *r, pwd),
            Op::Insert(f, t) => insert(*f, *t, pwd)
        }
    }
}

fn unscramble(pwd: &mut Vec<char>, ops: &[Op]) {
    for op in ops.iter().rev() {
        match op {
            Op::SwapPosition(x, y) => swap_pos(*x, *y, pwd),
            Op::SwapLetter(x, y) => swap_letter(*x, *y, pwd),
            Op::RotateLeft(n) => rot_right(*n, pwd),
            Op::RotateRight(n) => rot_left(*n, pwd),
            Op::RotateLetter(c) => rot_letter_rev(*c, pwd),
            Op::Reverse(l, r) => reverse(*l, *r, pwd),
            Op::Insert(f, t) => insert(*t, *f, pwd)
        }
    }
}

fn swap_pos(x: usize, y: usize, pwd: &mut Vec<char>) {
    let tmp = pwd[x];
    pwd[x] = pwd[y];
    pwd[y] = tmp;
}

fn swap_letter(x: char, y: char, pwd: &mut Vec<char>) {
    let mut x_marker = 0;
    let mut y_marker = 0;
    for (i, &c) in pwd.iter().enumerate() {
        if c == x {
            x_marker = i;
        } else if c == y {
            y_marker = i;
        }
    }
    pwd[x_marker] = y;
    pwd[y_marker] = x;
}

fn rot_right(n: usize, pwd: &mut Vec<char>) {
    let mut tmp = vec!['-'; pwd.len()];
    for i in 0..pwd.len() {
        tmp[(i + n) % pwd.len()] = pwd[i]
    }
    *pwd = tmp;
}

fn rot_left(n: usize, pwd: &mut Vec<char>) {
    let mut tmp = vec![];
    for i in n..(n + pwd.len()) {
        tmp.push(pwd[i % pwd.len()])
    }
    *pwd = tmp;
}

fn rot_letter(c: char, pwd: &mut Vec<char>) {
    for i in 0..pwd.len() {
        if pwd[i] == c {
            let n = if i >= 4 { i + 2 } else { i + 1 };
            rot_right(n, pwd);
            return;
        }
    }
}

fn rot_letter_rev(c: char, pwd: &mut Vec<char>) {
    for i in 0..pwd.len() {
        if pwd[i] == c {
            let dest = if i % 2 != 0 { i / 2 } 
                else if i == 0 { pwd.len() - 1 } 
                else  { i / 2 + (pwd.len() - 1) / 2 };

            let n = if dest <= i { i - dest } 
                else { (pwd.len() + i - dest) % pwd.len() };

            rot_left(n, pwd);
            return;
        }
    }
}

fn reverse(l: usize, r: usize, pwd: &mut Vec<char>) {
    let slice = &pwd[l..=r].to_owned();
    for (i, &c) in slice.iter().rev().enumerate() {
        pwd[l + i] = c;
    }
}

fn insert(from: usize, to: usize, pwd: &mut Vec<char>) {
    let c = pwd[from];
    pwd.remove(from);
    pwd.insert(to, c);
}

fn as_op(line: &str) -> Op {
    let split: Vec<_> = line.split_whitespace().collect();
    match split[0] {
        "swap" => {
            if split[1] == "position" {
                Op::SwapPosition(split[2].parse().unwrap(), split[5].parse().unwrap())
            } else {
                Op::SwapLetter(split[2].chars().next().unwrap(), split[5].chars().next().unwrap())
            }
        },
        "rotate" => {
            match split[1] {
                "left" => Op::RotateLeft(split[2].parse().unwrap()),
                "right" => Op::RotateRight(split[2].parse().unwrap()),
                "based" => Op::RotateLetter(split[6].chars().next().unwrap()),
                _ => panic!("Unrecognised rotate instruction.")
            }
        },
        "reverse" => {
            Op::Reverse(split[2].parse().unwrap(), split[4].parse().unwrap())
        },
        "move" => {
            Op::Insert(split[2].parse().unwrap(), split[5].parse().unwrap())
        },
        _ => panic!("Unrecognised instruction")
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    Reverse(usize, usize),
    Insert(usize, usize)
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}