use std::{time::Instant, collections::HashSet};

fn main() {
    let t = Instant::now();
    let mut bricks = include_str!("../../../input/22")
        .lines()
        .map(Brick::from_str)
        .collect::<Vec<_>>();

    let (p1, p2) = run(bricks);
    let e = t.elapsed().as_micros();

    println!("Part 1: {:6}", p1);
    println!("Part 2: {:6}", p2);
    println!("Time:   {e}us");
}

fn run(mut bricks: Vec<Brick>) -> (usize, usize) {
    // Sort such that the blocks lowest to the ground are in the list first.
    bricks.sort_by(|a, b| a.min_z().cmp(&b.min_z()));

    let mut rests = vec![];
    for i in 0..bricks.len() {
        let (b, resting_on) = fall(i, &bricks);
        bricks[i] = b;
        rests.push(resting_on);
    }

    let mut affects: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];
    for i in 0..bricks.len() {
        for j in 0..rests.len() {
            if i == j { continue }
            if rests[j].contains(&i) && rests[j].len() == 1 {
                affects[i].insert(j);
                for (k, r) in rests.iter().enumerate() {
                    if k == i { continue }
                    if r.len() > 0 && r.iter().all(|b| affects[i].contains(b)) {
                        affects[i].insert(k);
                    }
                }
            }
        }
    }

    let safe = affects.iter().filter(|h| h.len() == 0).count();
    let total_fall = affects.iter().map(|h| h.len()).sum::<usize>();

    (safe, total_fall)
}

/// Let a brick fall until it can't fall any further - return its new position
/// and the indexes of the bricks it now rests upon.
fn fall(i: usize, bricks: &[Brick]) -> (Brick, Vec<usize>) {
    let mut brick = bricks[i];
    let mut falling = true;
    let mut known_valid_pos = brick;
    let mut resting_on = vec![];

    while falling && brick.min_z() != 1 {
        brick = brick.move_down();
        for (j, b) in bricks.iter().enumerate() {
            if i == j { break }
            if brick.collides_with(&b) {
                falling = false;
                resting_on.push(j);
            }
        }
        if falling { known_valid_pos = brick; }
    }

    (known_valid_pos, resting_on)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Brick((usize, usize, usize), (usize, usize, usize));
impl Brick {
    pub fn from_str(s: &str) -> Self {
        let parse = |st: &str| st.split(',')
            .map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>();

        let (start, end) = s.split_once('~').unwrap();
        let (start, end) = (parse(start), parse(end));
        Brick((start[0], start[1], start[2]), (end[0], end[1], end[2]))
    }

    pub fn collides_with(&self, other: &Brick) -> bool {
        (self.max_x() >= other.min_x() && self.min_x() <= other.max_x())
        && (self.max_y() >= other.min_y() && self.min_y() <= other.max_y())
        && (self.max_z() >= other.min_z() && self.min_z() <= other.max_z())
    }

    pub fn move_down(&self) -> Brick {
        Brick(
            (self.0.0, self.0.1, self.0.2.saturating_sub(1)), 
            (self.1.0, self.1.1, self.1.2.saturating_sub(1))
        )
    }

    pub fn min_x(&self) -> usize { self.0.0.min(self.1.0) }
    pub fn max_x(&self) -> usize { self.0.0.max(self.1.0) }

    pub fn min_y(&self) -> usize { self.0.1.min(self.1.1) }
    pub fn max_y(&self) -> usize { self.0.1.max(self.1.1) }

    pub fn min_z(&self) -> usize { self.0.2.min(self.1.2) }
    pub fn max_z(&self) -> usize { self.0.2.max(self.1.2) }
}