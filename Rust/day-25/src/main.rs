fn main() {
    let input = include_str!("../../../input/25");
    let p1 = p1(input);
    println!("{}", p1);
}

fn to_digit(c: char) -> isize {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!()
    }
}

fn from_digit(n: isize) -> char {
    match n {
       4  =>  '2',
       3  =>  '1',
       2  =>  '0',
       1 =>  '-',
       0 =>  '=',
       _ => panic!()
    }
}

fn from_snafu(s: &str) -> isize {
    s.chars().rev().enumerate().fold(0, |n, (i, c)|  
        n + 5isize.pow(i as u32) * to_digit(c)
    )
}

fn as_snafu(mut n: isize) -> String {
    let mut out: Vec<isize> = vec![];
    while n > 0 {
        n += 2;
        out.push(n % 5);
        n /= 5;
    }
    out.into_iter().rev().map(from_digit).collect()
}

fn p1(s: &str) -> String {
    as_snafu(s.lines()
        .map(from_snafu)
        .sum())
}