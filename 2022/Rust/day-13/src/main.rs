use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
enum List { SubList(Vec<List>), Unit(u8) }
impl List {
    fn from_str(cs: &str) -> List {
        fn process(cs: &str) -> (List, usize) {
            let elems: Vec<_> = cs.chars().collect();
            let mut out = vec![];
            let mut i = 0;
            while i < elems.len() {
                if elems[i] == '[' {
                    let tmp: String = elems[(i + 1)..].iter().collect();
                    let (l, a) = process(&tmp);
    
                    i += a + 1;
                    out.push(l);
                } else if elems[i] == ']' {
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
        process(cs).0
    }
}

fn main() {
    let input = include_str!("../../../input/13");
    let (p1, p2) = (p1(input), p2(input));
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn p1(s: &str) -> usize {
    let mut count = 0;
    let pairs = s.split("\n\n");
    for (n, pair) in pairs.enumerate() {
        let strs: Vec<_> = pair.lines().map(List::from_str).collect();
        if compare(&strs[0], &strs[1]) == Ordering::Less { count += n + 1; }
    }
    count
}

fn p2(s: &str) -> usize {
    let mut lines: Vec<_> = s.lines()
        .filter(|line| !line.is_empty())
        .map(List::from_str)
        .collect();
    
    let dividers = ["[[2]]", "[[6]]"].map(List::from_str);
    dividers.iter().for_each(|d| lines.push(d.clone()));
    lines.sort_by(compare);
    lines.iter().enumerate().filter_map(|(i, l)| dividers.contains(l).then(|| (i + 1))).product()
}


fn compare(left: &List, right: &List) -> Ordering {
    match left {
        List::Unit(a) => match right {
            List::Unit(b) => a.cmp(b),
            List::SubList(_) => compare(&List::SubList(vec![List::Unit(*a)]), right)
        },
        List::SubList(a) =>  match right {
            List::Unit(b) => compare(left, &List::SubList(vec![List::Unit(*b)])),
            List::SubList(b) => {
                for (l, r) in a.iter().zip(b.iter()) {
                    match compare(l, r) {
                        Ordering::Equal => (),
                        result => return result
                    };
                }
                a.len().cmp(&b.len())
            }
        }
    }
}