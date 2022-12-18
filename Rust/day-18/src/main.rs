use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../../../input/18");
    let p1 = p1(input);
    let p2 = p2(input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Cube { x: isize, y: isize, z: isize }

impl Cube {
    pub fn new(x: isize, y: isize, z: isize) -> Self { Cube { x, y, z } }
    pub fn neighbours(&self) -> [Self; 6] {
        [
            Cube::new(self.x - 1, self.y, self.z),
            Cube::new(self.x + 1, self.y, self.z),
            Cube::new(self.x, self.y + 1, self.z),
            Cube::new(self.x, self.y - 1, self.z),
            Cube::new(self.x, self.y, self.z + 1),
            Cube::new(self.x, self.y, self.z - 1)
        ]
    }

    pub fn from_str(line: &str) -> Cube {
        let mut ns = line.trim().split(',').map(|n| n.parse().unwrap());
        Cube::new(ns.next().unwrap(), ns.next().unwrap(), ns.next().unwrap())
    }
}

fn p1(input: &str) -> usize {
    let cubes: HashSet<Cube> = HashSet::from_iter(input.lines().map(Cube::from_str));
    cubes.iter().map(|cube| cube.neighbours().iter()
        .filter(|neighbour| !cubes.contains(neighbour)).count()
    ).sum()
}

fn p2(input: &str) -> usize {
    let cubes: HashSet<Cube> = HashSet::from_iter(input.lines().map(Cube::from_str));
    let (mut lx, mut ly, mut lz) = (0, 0, 0);
    cubes.iter().for_each(|cube|
        (lx, ly, lz) = (lx.max(cube.x), ly.max(cube.y), lz.max(cube.z))
    );
    let end = Cube::new(lx + 2, ly + 2, lz + 2);
    flood(Cube::new(-2, -2, -2), &cubes, &mut HashSet::new(), end)
}

fn flood(s: Cube, map: &HashSet<Cube>, visited: &mut HashSet<Cube>, end: Cube) -> usize {
    let mut queue = VecDeque::new();
    let mut sum = 0;

    queue.push_back(s);
    while !queue.is_empty() {
        let start = queue.pop_front().unwrap();
        if !(visited.contains(&start) || start.x > end.x || start.x < s.x || start.y > end.y
            || start.y < s.y || start.z > end.z || start.z < s.z) 
        { 
            visited.insert(start);
            start.neighbours().into_iter().for_each(|n|
                if map.contains(&n) { sum += 1; } else { queue.push_back(n); }
            );
        }
    }
    sum
}