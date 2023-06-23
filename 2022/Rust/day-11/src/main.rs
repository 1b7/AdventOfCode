fn main() {
    let input = include_str!("../../../input/11");
    let (p1, p2) = run(input);
    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
}

#[derive(Debug, Clone, Copy)]
enum Operator { Mul, Add }

#[derive(Debug, Clone, Copy)]
enum Operand {Int(usize), Old}

#[derive(Debug, Clone, Copy)]
struct Operation { left: Operand, right: Operand, op: Operator }
impl Operation {
    pub fn operate(&self, old: usize) -> usize {
        let unbox = |o| match o { Operand::Old =>  old, Operand::Int(x) => x };
        match self.op {
            Operator::Mul => unbox(self.left) * unbox(self.right),
            Operator::Add => unbox(self.left) + unbox(self.right)
        }
    }
}

#[derive(Debug, Clone)]
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
            let mut result = self.operation.operate(item.unwrap());
            result = if p == 0 { result / 3 } else { result %  p };
            
            let throw_to = if result % self.test.0 == 0 { self.test.1 } else { self.test.2 };
            out.push((result, throw_to));
            self.inspections += 1;
            item = self.items.pop();
        }
        out

    }
    
    fn from_str(txt: &str) -> Monkey {
        let fields: Vec<_> = txt.split('\n').collect();
        let items: Vec<_> = fields[1].split_once(':').unwrap().1.split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let ops: Vec<_> = fields[2].split_once("= ").unwrap().1.split(' ').collect();
        let (left, mid, right) = (ops[0], ops[1], ops[2]);
        let left = left.parse().map_or_else(|_| Operand::Old, Operand::Int);
        let right = right.parse().map_or_else(|_| Operand::Old, Operand::Int);
        let mid = match mid { "*" => Operator::Mul, _ => Operator::Add };
        let operation = Operation { left, right, op: mid };
    
        let test: usize = fields[3].split_once("by ").unwrap().1.parse().unwrap();
        let test_true: usize = fields[4].split_once("monkey ").unwrap().1.parse().unwrap();
        let test_false: usize = fields[5].split_once("monkey ").unwrap().1.parse().unwrap();
    
        Monkey {
            items, operation, test: (test, test_true, test_false), inspections: 0
        }
    }
}

fn run(input: &str) -> (usize, usize) {
    let mut monkeys: Vec<_> = input.split("\n\n").map(Monkey::from_str).collect();
    let p = monkeys.iter().map(|m| m.test.0).product();
    (px(&mut monkeys.clone(), 20, 0), px(&mut monkeys, 10_000, p))
}

fn px(monkeys: &mut [Monkey], n: usize, p: usize) -> usize {
    for _ in 0..n {
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