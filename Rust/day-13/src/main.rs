use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
enum List {
    SubList(Vec<List>),
    Unit(u8)
}

impl List {
    fn from_str(cs: &str) -> (List, usize) {
        let elems: Vec<_> = cs.chars().collect();
        let mut out = vec![];
        let mut i = 0;
        while i < elems.len() {
            if elems[i] == '[' {
                let tmp: String = elems[(i + 1)..].iter().collect();
                let (l, a) = List::from_str(&tmp);

                i += a + 1;
                out.push(l);
            }
            else if elems[i] == ']' {
                let right: String = elems[i..].iter().take_while(|&&c| c != ']').collect();
                if !right.is_empty() {
                    out.push(List::Unit(right.parse().unwrap()));
                }
                return (List::SubList(out), i);
            } else {
                let num: String = elems[i..].iter().take_while(|c| c.is_ascii_digit()).collect();
                if !num.is_empty() {
                    out.push(List::Unit(num.parse().unwrap()));
                }
            }
            i += 1;
        }
        (List::SubList(out), i)
    }
}

fn main() {
    let input = include_str!("../../../input/13");
    let p1 = p1(input);
    let p2 = p2(input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn p1(s: &str) -> usize {
    let mut count = 0;
    let pairs = s.split("\n\n");
    for (n, pair) in pairs.enumerate() {
        let (left, right) = pair.split_once('\n').unwrap();
        let (l, _) = List::from_str(left);
        let (r, _) = List::from_str(right);
        if compare(&l, &r) == Ordering::Less { count += n + 1; }
    }
    count
}

fn p2(s: &str) -> usize {
    let mut lines: Vec<_> = s.lines()
        .filter(|line| !line.is_empty())
        .map(|line| List::from_str(line).0)
        .collect();
    
    let dividers: Vec<_> = ["[[2]]", "[[6]]"].iter().map(|x| List::from_str(x).0).collect();
    lines.append(&mut dividers.clone());
    lines.sort_by(compare);

    let mut prod = 1;
    for (i, line) in lines.iter().enumerate() {
        if dividers.contains(line) {
            prod *= i + 1;
        }
    }
    prod
}


fn compare(left: &List, right: &List) -> Ordering {
    match left {
        List::Unit(a) => {
            match right {
                List::Unit(b) => a.cmp(b),
                List::SubList(_) => compare(&List::SubList(vec![List::Unit(*a)]), right)
            }
        },
        List::SubList(a) => {
            match right {
                List::SubList(b) => {
                    for (l, r) in a.iter().zip(b.iter()) {
                        let result = compare(l, r);
                        if result != Ordering::Equal {
                            return result;
                        }
                    }
                    a.len().cmp(&b.len())
                },
                List::Unit(b) => compare(left, &List::SubList(vec![List::Unit(*b)]))
            }
        }
    }
}