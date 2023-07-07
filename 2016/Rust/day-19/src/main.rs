fn main() {
    let n: usize = load_input().parse().expect("Invalid number passed in.");
    println!("Part 1: {}", elephant(n));
    println!("Part 2: {}", fast_opposite(n));
}

fn elephant(n: usize) -> usize {
    fn find_next(mut from: usize, elves: &[u16]) -> usize {
        if from == elves.len() { from = 0; } 
        while elves[from] != 1 { 
            from += 1; 
            if from == elves.len() { from = 0; } 
        }
        from
    }

    let mut elves = vec![1u16; n];
    let mut i = 0;
    let mut last_i = 0;
    for _ in 0..(n - 1) {
        i = find_next(i, &elves);
        let j = find_next(i + 1, &elves);
        elves[j] = 0;
        last_i = i;
        i = j + 1;
    }
    last_i + 1
}

// This function is based on problem analysis, see below.
fn fast_opposite(n: usize) -> usize {
    let n = n as f64;
    (((n) - (3.0f64).powf(n.log(3.0).floor()))) as usize
}

// NOTE: The following function for Part 2 is a solution in principle, however
// it is extremely slow for large values of n.
//
// Instead, this can be solved analytically by observing a pattern;
// removing opposite elves causes the index to effectively count up in powers of
// 3 - after it hits that 3^n, it starts back from 1 - and for odd powers, skips
// even numbers.
//
// e.g. for an input of 12345, floor(log_3(12345)) = 8.
// 3^8 = 6561, meaning we would need to count a further 5784. This is our answer.
fn opposite(n: u32) -> usize {
    let mut rem = n;
    let mut i = 0;
    let mut elves = (0..n).map(|x| {
        if x == 0 {
            Node::new(n - 1, x, 1)
        } else if x == (n - 1) {
            Node::new(x - 1, x, 0)
        } else {
            Node::new(x - 1, x, x + 1)
        }
    }).collect::<Vec<Node>>();

    for _ in 0..(n - 1) {
        let current = elves[i];

        let mut next = current;
        for _ in 0..(rem / 2) {
            next = elves[next.next as usize];
        }
        
        // 'Remove' `next`
        elves[next.prev as usize].next = next.next;
        elves[next.next as usize].prev = next.prev;

        // i -> next neighbour left.
        i = elves[current.cur as usize].next as usize;
        rem -= 1;
    }
    i + 1
}

#[derive(Debug, Clone, Copy)]
struct Node {prev: u32, cur: u32, next: u32}
impl Node {
    pub fn new(prev: u32, cur: u32, next: u32) -> Node { Self { prev, cur, next } }
}

fn load_input() -> String {
    std::fs::read_to_string(
        std::env::args().nth(1).expect("Filepath not provided.")
    ).expect("Could not load file!")
}
