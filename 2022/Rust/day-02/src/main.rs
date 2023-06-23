fn main() {
    println!("Part 1: {}", p1());
    println!("Part 2: {}", p2());
}

fn p1() -> usize {
    let input = include_str!("../../../Input/02");

    let mut score = 0;
    for line in input.lines() {
        let mut s = line.split_whitespace().take(2);

        let o = s.next().unwrap();
        let p = s.next().unwrap();
        
        score += match p {
            "X" => 1 + match o {
                "A" => 3,
                "B" => 0,
                _ => 6
            },
            "Y" => 2 + match o {
                "A" => 6,
                "B" => 3,
                _ => 0
            },
            "Z" => 3 + match o {
                "A" => 0,
                "B" => 6,
                _ => 3
            },
            _ => panic!()
        };

    }
    score
}



fn p2() -> usize {
    let input = include_str!("../../../Input/02");

    let mut score = 0;
    for line in input.lines() {
        let mut s = line.split_whitespace().take(2);

        let o = s.next().unwrap();
        let p = s.next().unwrap();
        
        score += match p {
            "X" => 0 + match o {
                "A" => 3,
                "B" => 1,
                _ => 2
            },
            "Y" => 3 + match o {
                "A" => 1,
                "B" => 2,
                _ => 3
            },
            "Z" => 6 + match o {
                "A" => 2,
                "B" => 3,
                _ => 1
            },
            _ => panic!()
        };
    }
    score
}