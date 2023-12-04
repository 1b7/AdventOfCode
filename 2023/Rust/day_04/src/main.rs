use std::time::Instant;

fn main() {
    let mut cards = setup(include_str!("../../../input/04"));

    let s = Instant::now();
    let p1 = p1(&cards);
    let t1 = s.elapsed().as_micros();

    let s = Instant::now();
    let p2 = p2(&mut cards);
    let t2 = s.elapsed().as_micros();

    println!("Part 1: {:8} ({}us)\nPart 2: {:8} ({}us)", p1, t1, p2, t2);
}

fn setup(s: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    s.lines().map(|line| {
        let (win, have) = line.split_once('|').unwrap();
        let win = win.split_once(':').unwrap().1.trim();

        let win = win.split_whitespace().map(|n| n.parse::<u8>().unwrap()).collect::<Vec<_>>();
        let mut have = have.trim().split_whitespace().map(|n| n.parse::<u8>().unwrap()).collect::<Vec<_>>();

        have.sort();

        (win, have)
    }).collect()
}

fn card_matches(card: &(Vec<u8>, Vec<u8>)) -> u32 {
    card.0.iter().fold(0, |acc, h| if card.1.binary_search(h).is_ok() {acc + 1} else { acc } )
}

fn p1(cards: &Vec<(Vec<u8>, Vec<u8>)>) -> u32 {
    cards.iter().fold(0, |sum, card| {
        let matches = card_matches(&card);
        if matches > 0 { sum + (1 << (matches - 1)) } else { sum }
    })
}

fn p2(cards: &Vec<(Vec<u8>, Vec<u8>)>) -> u32 {
    let mut cards: Vec<(&(Vec<u8>, Vec<u8>), u32)> = cards.iter().map(|card| (card, 1)).collect();
    let mut new_len = 0;
    for i in 0..cards.len() {
        let (card, n) = cards[i];
        new_len += n;

        let matches = card_matches(&card) as usize;
        for m in 1..=matches { cards[i + m].1 += n;  }
    }
    new_len
}