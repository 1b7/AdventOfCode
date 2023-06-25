fn main() {
    let instructions: Vec<Vec<char>> = load_input().lines()
        .map(|line| line.chars().collect()).collect();

    println!("Part 1: {}", p1(&instructions));
    println!("Part 2: {}", p2(&instructions));
    
}

fn p1(ins: &[Vec<char>]) -> i32 {
    let clamp = |n, min, max| if n < min { min } else if n > max { max } else { n };
    let char_to_change = |c| match c {
        'U' => ( 0, -1),
        'D' => ( 0,  1),
        'L' => (-1,  0),
        'R' => ( 1,  0),
         _  => panic!("Unrecognised Character")
    };

    let mut output = 0;
    let (mut x, mut y) = (1, 1);
    for row in ins {
        for &chr in row {
            let (dx, dy) = char_to_change(chr);
            (x, y) = (clamp(x + dx, 0, 2), clamp(y + dy, 0, 2));
        }
        output *= 10;
        output += (3 * y) + (x + 1);
    }
    output
}

fn p2(ins: &[Vec<char>]) -> String {
    let mut board = vec![0; 7 * 7];
    for (i, n) in [
                10, 
            16, 17, 18, 
        22, 23, 24, 25, 26, 
            30, 31, 32, 
                38
    ].iter().enumerate() {
        board[*n] = (i + 1) as u8;
    }

    let mut output = String::new();
    let mut index = 22;
    for row in ins {
        for dir in row {
            let adjust: i32 = match dir {
                'U' => -7,
                'D' =>  7,
                'L' => -1,
                'R' =>  1,
                 _  => panic!("Unrecognised Character")
            };
            if board[(index + adjust) as usize] != 0 { index += adjust };
        }
        output.push_str(&format!("{:x}", board[index as usize]));
    }
    output.to_ascii_uppercase()
}


fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}