use std::{str::FromStr, fmt::Error, collections::HashMap, cmp::Ordering};
use itertools::{self, Itertools};

fn main() {
    let input = include_str!("../../../input/21");
    let mut monkeys = HashMap::new();
    input.lines().map(as_monkey).for_each(|(id, op)| {
        monkeys.insert(id, op);
    });
    let p1 = traverse("root", &monkeys);

    let p2 = p2(&mut monkeys);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[derive(Debug, Clone, Copy)]
enum Operand { Add, Mul, Sub, Div }
impl Operand {
    pub fn exec(&self, l: f64, r: f64) -> f64 {
        match &self {
            Self::Add => l + r,
            Self::Mul => l * r,
            Self::Div => l / r,
            Self::Sub => l - r
        }
    }
}

impl FromStr for Operand {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operand::Add),
            "*" => Ok(Operand::Mul),
            "-" => Ok(Operand::Sub),
            "/" => Ok(Operand::Div),
            _   => panic!()
        }
    }
}

#[derive(Debug, Clone)]
enum Op { 
    Action(Operand, String, String),
    Number(f64)
}

fn as_monkey(line: &str) -> (String, Op) {  
    fn as_operation(s: &str) -> Op {
        let s = s.trim();
        if s.chars().all(char::is_numeric) {
            Op::Number(s.parse().unwrap())
        } else {
            let (l, o, r) = s.split_whitespace().take(3).collect_tuple().unwrap();
            Op::Action(Operand::from_str(o).unwrap(), l.to_string(), r.to_string())
        }
    }
    let (id, op) = line.split_once(':').unwrap();
    (id.to_string(), as_operation(op))
}

fn p2(monkeys: &mut HashMap<String, Op>) -> f64 {
    let (root_l, root_r) = match monkeys.get("root").unwrap().clone() {
        Op::Number(_) => panic!(),
        Op::Action(_, l, r) => (l ,r)
    };

    let left = traverse(&root_l, &monkeys);
    let right = traverse(&root_r, &monkeys);
    // Test an arbitrary change: if it affects the result of the left, 'humn' must
    // be on the lhs of the tree, else the rhs.
    if let Op::Number(n) = monkeys.get_mut("humn").unwrap() { *n -= 1.0; }

    let human_on_right =  left == traverse(&root_l, &monkeys);
    let target = if human_on_right { left } else { right };
    let from = if human_on_right { root_r } else { root_l };

    // Binary search over possible solutions until we find one which matches:
    let mut low = 0.0;
    let mut high = 10_000_000_000_000.0;
    loop {
        let human_val = ((low + high) / 2.0f64).floor();
        if let Op::Number(n) = monkeys.get_mut("humn").unwrap() {
            *n = human_val;
        } else { panic!() }

        match traverse(&from, &monkeys).total_cmp(&target) {
            Ordering::Greater => low = human_val + 1.0,
            Ordering::Less => high = human_val - 1.0,
            Ordering::Equal => return human_val
        }
    }
}

fn traverse(from: &str, list: &HashMap<String, Op>) -> f64 {
    match list.get(from).unwrap().clone() {
        Op::Number(x) => x,
        Op::Action(ac, l, r) =>  ac.exec(traverse(&l, list), traverse(&r, list))
    }
}