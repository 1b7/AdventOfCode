fn main() {
    let input = include_str!("../../../input/11");
    println!("Part 1: {:?}", p1(input));
    println!("Part 2: {:?}", p2(input));
}

#[derive(Debug)]
enum Operator { Mul, Add }
#[derive(Debug)]
enum Operand {Int(usize), Old}

#[derive(Debug)]
struct Operation {
    left: Operand,
    right: Operand,
    op: Operator
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: (usize, usize, usize),
    inspections: usize
}

impl Monkey {
    fn inspect(&mut self, p: usize) -> Vec<(usize, usize)> {
        let mut item = self.items.pop();
        let mut out = vec![];

        while item.is_some() {
            let item_int = item.unwrap();
            let left = match &self.operation.left {
                Operand::Int(x) => *x,
                _ => item_int
            };
            let right = match &self.operation.right {
                Operand::Int(x) => *x,
                _ => item_int
            };
            let mut result = match self.operation.op {
                Operator::Add => left + right,
                _ => left * right
            };

            
            result = if p == 0 { result / 3 } else { result %  p };
            let throw_to = if result % self.test.0 == 0 { self.test.1 } else { self.test.2 };

            out.push((result, throw_to));
            self.inspections += 1;
            item = self.items.pop();
        }
        out

    }
}

fn str_as_monkey(txt: &str) -> Monkey {
    let fields: Vec<_> = txt.split('\n').collect();
    let items: Vec<_> = fields[1].split_once(':').unwrap().1.split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();
    let op = fields[2].split_once("= ").unwrap().1;
    let ops: Vec<_> = op.split(' ').collect();
    let (left, mid, right) = (ops[0], ops[1], ops[2]);
    // println!("{} {} {}", left, mid, right);
    let left = left.parse().map_or_else(|_| Operand::Old, |x| Operand::Int(x));
    let right = right.parse().map_or_else(|_| Operand::Old, |x| Operand::Int(x));
    let mid = match mid {
        "*" => Operator::Mul,
        _ => Operator::Add
    };
    let operation = Operation { left, right, op: mid };

    let test: usize = fields[3].split_once("by ").unwrap().1.parse().unwrap();
    let test_true: usize = fields[4].split_once("monkey ").unwrap().1.parse().unwrap();
    let test_false: usize = fields[5].split_once("monkey ").unwrap().1.parse().unwrap();

    Monkey {
        items, operation, test: (test, test_true, test_false), inspections: 0
    }
}

fn p1(input: &str) -> usize {
    let monkeys = input.split("\n\n");
    let mut monkeys: Vec<_> = monkeys.map(str_as_monkey).collect();

    for _ in 0..20 {
        // println!("{:#?}", monkeys);
        for i in 0..monkeys.len() {
            let out = monkeys[i].inspect(0);
            for (x, n) in out {
                monkeys[n].items.push(x);
            }
        }
    }
    let mut ins: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    ins.sort_by(|a, b| b.cmp(a));
    ins[0..2].iter().product()
}

fn p2(input: &str) -> usize {
    let monkeys = input.split("\n\n");
    let mut monkeys: Vec<_> = monkeys.map(str_as_monkey).collect();

    let p = monkeys.iter().map(|m| m.test.0).product();

    for r in 0..10_000 {
        for i in 0..monkeys.len() {
            let out = monkeys[i].inspect(p);
            for (x, n) in out {
                monkeys[n].items.push(x);
            }
        }
    }
    let mut ins: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    ins.sort_by(|a, b| b.cmp(a));
    ins[0..2].iter().product()
}