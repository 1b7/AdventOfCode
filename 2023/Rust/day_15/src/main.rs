use std::time::Instant;

fn main() {
    let input = include_str!("../../../input/15");

    let s = Instant::now();
    let p1 = input.split(',')
        .map(hash)
        .sum::<usize>();

    let e = s.elapsed().as_micros();
    println!("Part 1: {p1} ({e}us)");

    let s = Instant::now();
    let p2 = p2(input);
    let e = s.elapsed().as_micros();
    println!("Part 2: {p2} ({e}us)");
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, x| ((acc + (x as usize)) * 17) % 256 )
}

fn p2(s: &str) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    
    s.split(',').for_each(|s| {
        let mut label = vec![];
        let mut rest = vec![];
        s.chars().for_each(|c| if c.is_alphabetic() { label.push(c) } else { rest.push(c) } );
        let label = String::from_iter(&label);
        let hash = hash(&label);

        if rest[0] == '-' {
            if let Some((i, _)) = boxes[hash].iter().enumerate().find(|p| p.1.0 == label) {
                boxes[hash].remove(i);
            }
        } else {
            let fl = String::from_iter(&rest[1..]).parse().unwrap();
            if let Some((i, _)) = boxes[hash].iter().enumerate().find(|p| p.1.0 == label) {
                boxes[hash][i].1 = fl;
            } else {
                boxes[hash].push((label, fl))
            }
        };
    });

    boxes.iter().enumerate().fold(0, |acc, (box_no, b)| {
        acc + b.iter().enumerate().map(|(slot, lens)| 
            (1 + box_no) * (slot + 1) * lens.1
        ).sum::<usize>()
    })
}