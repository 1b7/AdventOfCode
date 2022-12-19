use std::{str::FromStr, io::Error};
use std::collections::{VecDeque, HashSet};
use std::time::Instant;

fn main() {
    let input = include_str!("../../../input/19");
    let s = Instant::now();
    let p1 = p1(input);
    let e = s.elapsed();
    println!("Part 1: {}", p1);
    println!("Took: {}", e.as_millis());
}


fn p1(input: &str) -> usize {
    let factories: Vec<_> = input.lines().map(|s| Factory::from_str(s).unwrap()).collect();
    // let results = run(factories[1]);
    let mut sum = 0;
    let t = Instant::now();
    for (i, &factory) in factories.iter().enumerate() {
        println!("{}/{} Complete\t[{:.4}s]", i, factories.len(), t.elapsed().as_secs_f32());
        sum += run(factory);
    }
    println!("All Complete\t[{:.4}s]", t.elapsed().as_secs_f32());
    // let results: Vec<_> = factories.into_iter().map(run).collect();
    // dbg!(&results);
    // results.into_iter().sum()
    // results
    sum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Mat { Ore, Clay, Obsidian, Geode }

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Factory {
    id: u8,
    costs: [(u8, u8); 4],
    mats: [usize; 4],
    bots: [usize; 4],
    building: Option<u8>
    // Format: [ORE, CLAY, OBSIDIAN, GEODE]
}

impl Factory {
    pub fn new(id: u8, costs: [(u8, u8); 4]) -> Self {
        Factory { id, costs, mats: [0; 4], bots: [1, 0, 0, 0], building: None }
    }

    pub fn build_ore(&mut self) {
        self.mats[Mat::Ore as usize] -= self.costs[Mat::Ore as usize].0 as usize;
        // self.bots[Mat::Ore as usize] += 1;
        self.building = Some(Mat::Ore as u8);
    }

    pub fn build_clay(&mut self) {
        self.mats[Mat::Ore as usize] -= self.costs[Mat::Clay as usize].0 as usize;
        // self.bots[Mat::Clay as usize] += 1;
        self.building = Some(Mat::Clay as u8);
    }

    pub fn build_obsidian(&mut self) {
        self.mats[Mat::Ore as usize] -= self.costs[Mat::Obsidian as usize].0 as usize;
        self.mats[Mat::Clay as usize] -= self.costs[Mat::Obsidian as usize].1 as usize;
        // self.bots[Mat::Obsidian as usize] += 1;
        self.building = Some(Mat::Obsidian as u8);
    }

    pub fn build_geode(&mut self) {
        self.mats[Mat::Ore as usize] -= self.costs[Mat::Geode as usize].0 as usize;
        self.mats[Mat::Obsidian as usize] -= self.costs[Mat::Geode as usize].1 as usize;
        // self.bots[Mat::Geode as usize] += 1;
        self.building = Some(Mat::Geode as u8);
    }

    pub fn build_robot(&mut self, i: Mat) {
        match i {
            Mat::Ore => self.build_ore(),
            Mat::Clay => self.build_clay(),
            Mat::Obsidian => self.build_obsidian(),
            Mat::Geode => self.build_geode()
        };
    }

    pub fn harvest(&mut self) {
        self.mats[Mat::Ore as usize]      += self.bots[Mat::Ore as usize];
        self.mats[Mat::Clay as usize]     += self.bots[Mat::Clay as usize];
        self.mats[Mat::Obsidian as usize] += self.bots[Mat::Obsidian as usize];
        self.mats[Mat::Geode as usize]    += self.bots[Mat::Geode as usize];

        if self.building.is_some() { self.bots[self.building.unwrap() as usize] += 1 }
        self.building = None;
    }

    pub fn quality(&self) -> usize { self.id as usize * self.mats[Mat::Geode as usize] }
}

impl FromStr for Factory {
    fn from_str(s: &str) -> Result<Self, Error> {
        let (id_str, rest) = s.split_once(':').unwrap();
        let id = id_str.chars()
            .filter(char::is_ascii_digit).collect::<String>()
            .parse().unwrap();
        
        let mut costs = [(0, 0); 4];
        let costs_str = rest.split("costs ").skip(1);
        for (i, cost) in costs_str.enumerate() {
            let l = cost.chars()
                .take_while(char::is_ascii_digit).collect::<String>()
                .parse().unwrap();
            let r = cost.chars()
                .skip(3)
                .skip_while(|c| c.is_alphabetic() || c.is_whitespace())
                .take_while(char::is_ascii_digit).collect::<String>()
                .parse().unwrap_or(0);
            costs[i] = (l, r);
        }

        Ok(Factory::new(id, costs))

    }
    type Err = Error;
}

fn sum_up(n: usize) -> Vec<usize> {
    (0..=n).scan(0usize, |acc, x| { *acc += x; Some(*acc) }).collect()
}

fn run(factory: Factory) -> usize {
    let mut max_quality = 0;
    let mut max_geodes = 0;
    let mut fs = VecDeque::new();
    fs.push_back((factory, vec![], 24));

    let mut memo = HashSet::new();

    let max_possible_additions = sum_up(24);

    while !fs.is_empty() {
        let (mut f, f_order, time) = fs.pop_front().unwrap();
        let time = time - 1;
        f.harvest();

        if f.quality() > max_quality {
            max_geodes = f.mats[Mat::Geode as usize];
            max_quality = max_quality.max(f.quality());
            // println!("{:?}", f_order);
            // dbg!(f.id, f.mats[Mat::Geode as usize], time);
        }
        if time == 0 { continue; }

        // Decision Tree Pruning:
        if f_order.len() > 8 && memo.contains(&f_order.clone())  { 
            continue; 
        }
        memo.insert(f_order.clone());

        if f.quality() < max_quality {
            continue; 
        }

        if f.mats[Mat::Geode as usize] + max_possible_additions[time] <= max_geodes {
            continue;
        }

        for m in [Mat::Geode, Mat::Obsidian, Mat::Clay, Mat::Ore] {
            let (cost_a, cost_b) = f.costs[m as usize];

            let affordable = match m {
                Mat::Ore =>      f.mats[Mat::Ore as usize] >= cost_a as usize,
                Mat::Clay =>     f.mats[Mat::Ore as usize] >= cost_a as usize,
                Mat::Obsidian => f.mats[Mat::Ore as usize] >= cost_a as usize
                                 && f.mats[Mat::Clay as usize] >= cost_b as usize,
                Mat::Geode =>    f.mats[Mat::Ore as usize] >= cost_a as usize 
                                 && f.mats[Mat::Obsidian as usize] >= cost_b as usize,
            };

            if affordable {
                let mut nf = f;
                nf.build_robot(m);

                let mut order = f_order.clone();
                order.push(m as u8);

                fs.push_back((nf, order, time));
            }
        }
        fs.push_back((f, f_order.clone(), time));
    }
    dbg!(max_quality);
    max_quality
}