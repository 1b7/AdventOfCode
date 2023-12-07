use std::{time::Instant, collections::HashSet};

fn main() {
    let input = include_str!("../../../input/07");

    let s1 = Instant::now();
    let p1 = p1(input);
    let e1 = s1.elapsed().as_micros();
    
    let s2 = Instant::now();
    let p2 = p2(input);
    let e2 = s2.elapsed().as_micros();

    println!("Part 1: {p1} ({e1}us)\nPart 2: {p2} ({e2}us)");
}

fn p1(input: &str) -> usize {
    fn card_value(c: char) -> u8 {
        match c {
            '2'..='9' => c as u8 - b'2',
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => unreachable!("Invalid card value passed in")
        }
    }
    
    fn as_hand(s: &str) -> [u8; 5] {
        s.chars().map(card_value).take(5).collect::<Vec<_>>().try_into().unwrap()
    }
    
    let mut hands = input.lines().map(|l| {
        let l = l.split_once(' ').unwrap();
        let hand = as_hand(l.0);
        let hand_type = hand_type(&hand);
        (hand_type, hand, l.1.parse::<u32>().unwrap())
    }).collect::<Vec<_>>();

    hands.sort();

    let mut total = 0;
    for (i, &(_, _, bid)) in hands.iter().enumerate() {
        total += (i + 1) *( bid as usize)
    }
    
    total
}

fn hand_type(hand: &[u8; 5]) -> HandType {
    let mut counts = [0; 13];
    hand.iter().for_each(|&card| counts[card as usize] += 1);
    counts.sort_by(|a, b| b.cmp(a));
        
    match (counts[0], counts[1]) {
        (5, _) => HandType::FiveKind,
        (4, _) => HandType::FourKind,
        (3, 2) => HandType::FullHouse,
        (3, _) => HandType::ThreeKind,
        (2, 2) => HandType::TwoPair,
        (2, _) => HandType::Pair,
        (1, _) => HandType::HighCard,
        _      => unreachable!("No valid hand type found!")
    }

}

fn p2(input: &str) -> usize {
    fn card_value(c: char) -> u8 {
        match c {
            'J' => 0,
            '2'..='9' => (c as u8 - b'2') + 1,
            'T' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => unreachable!("Invalid card value passed in")
        }
    }
    
    fn as_hand(s: &str) -> [u8; 5] {
        s.chars().map(card_value).take(5).collect::<Vec<_>>().try_into().unwrap()
    }
    
    fn hand_strength(hand: &[u8; 5]) -> HandType {
        // Try turning the joker into every other card in the hand in turn:
        hand.iter().map(|&n| {
            let p: [u8; 5] = hand.iter().map(|&x| if x == 0 { n } else { x })
                .collect::<Vec<_>>().try_into().unwrap();
            hand_type(&p)
        }).max().unwrap()       
    }
    
    let mut hands = input.lines().map(|l| {
        let l = l.split_once(' ').unwrap();
        let hand = as_hand(l.0);
        let hand_type = hand_strength(&hand);
        (hand_type, hand, l.1.parse::<u32>().unwrap())
    }).collect::<Vec<_>>();

    hands.sort();

    let mut total = 0;
    for (i, &(_, _, bid)) in hands.iter().enumerate() {
        total += (i + 1) *( bid as usize)
    }
    
    total
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType { HighCard, Pair, TwoPair, ThreeKind, FullHouse, FourKind, FiveKind }
