fn main() {
    println!("Part One: {}", p1());
    println!("Part Two: {}", p2());
}

fn p1() -> u64 {
    let input = include_str!("../../../../input/2025/06")
        .lines()
        .map(|line| line.split_whitespace())
        .collect::<Vec<_>>();

    let nums_length = input.len() - 1;

    let nums = input[..nums_length]
        .iter()
        .map(|ns| ns.clone().map(|n| n.parse::<u64>().expect("Not a number")).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ops = input[nums_length]
        .clone()
        .map(Op::from_str)
        .collect::<Vec<_>>();


    let mut total = 0;
    for col in 0..nums[0].len() {
        let mut result = ops[col].identity();
        for  row in 0..nums.len() {
            result = ops[col].apply(result, nums[row][col])
        }
        total += result;
    }

    total
}

fn p2() -> u64 {
    let input = include_str!("../../../../input/2025/06")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let op_row = input.len() - 1;
    let max_len = input.iter().map(|row| row.len()).max().unwrap();

    let mut groups = vec![];
    let mut group = vec![];

    for col in 0..max_len {
        let mut num = 0;
        for row in 0..op_row {
            if col >= input[row].len() { continue; }
            match input[row][col].to_digit(10) {
                Some(n) => { num = num * 10 + (n as u64) },
                None => (),
            }
        }

        if num == 0 {
            groups.push(group);
            group = vec![];
        } else {
            group.push(num);
        }
    }
    groups.push(group);

    let ops = input[op_row]
        .iter()
        .filter(|c| !c.is_whitespace())
        .map(Op::from_char)
        .collect::<Vec<_>>();


    groups.iter().zip(ops).map(|(group, op)| {
        group.iter().fold(op.identity(), |acc, &num| { op.apply(acc, num) })
    }).sum()
}

#[derive(Debug, Clone, Copy)]
enum Op { Add, Mul }

impl Op {
    fn identity(&self) -> u64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }

    fn apply(&self, a: u64, b: u64,) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b
        }
    }

    fn from_char(c: &char) -> Self {
        match c {
            '+' => Op::Add,
            '*' => Op::Mul,
            _ => panic!("Unrecognised operation"),
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("Unrecognised operation"),
        }
    }
}
