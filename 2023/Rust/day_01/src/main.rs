fn main() {

    let input = include_str!("../../../input/01")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let p1 = input.iter()
        .map(|l| {
            let mut n1: Option<i32> = None;
            let mut nl: Option<i32> = None;

            for i in 0..l.len() {
                let x = parse_digit(&l[i..i+1].iter().collect::<String>(), false);
                if x.is_some() && n1.is_none() { n1 = x; }
                if x.is_some() { nl = x; }
            }

            return n1.unwrap() * 10 + nl.unwrap(); 
        }).sum::<i32>();
    println!("Part 1: {}", p1);

    let p2 = input.iter()
        .map(|l| {
            let mut n1: Option<i32> = None;
            let mut nl: Option<i32> = None;

            for i in 0..l.len() {
                for j in [1,3,4,5] {
                    if i + j > l.len() { continue; }
                    let x = parse_digit(&l[i..(i + j)].iter().collect::<String>(), true);
                    if x.is_some() && n1.is_none() { n1 = x; }
                    if x.is_some() { nl = x; }
                }
            }

            return n1.unwrap() * 10 + nl.unwrap(); 
        }).sum::<i32>();
    println!("Part 2: {}", p2);
}

fn parse_digit(s: &str, accept_words: bool) -> Option<i32> {
    if s.len() == 1 { 
        let p = s.parse::<i32>();
        if p.is_err() {  return None } else { return Some(p.unwrap()) }
    }

    if accept_words {
        match s {
            "zero" => Some(0),
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => None
        }
    } else { None }

}