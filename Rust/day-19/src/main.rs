use std::{str::FromStr, io::Error};
use std::collections::{VecDeque, HashSet};
use std::time::Instant;

fn main() {
    let input = include_str!("../../../input/19");
    let s = Instant::now();
    // let p1 = p1(input);
    let p2 = p2(input);
    let e = s.elapsed();
    // println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Took: {}", e.as_millis());
}


fn p1(input: &str) -> usize {
    let factories: Vec<_> = input.lines().map(|s| Factory::from_str(s).unwrap()).collect();
    let mut sum = 0;
    let t = Instant::now();
    for (i, &factory) in factories.iter().enumerate() {
        println!("{}/{} Complete\t[{:.4}s]", i, factories.len(), t.elapsed().as_secs_f32());
        sum += run(factory, 24).0;
    }
    println!("All Complete\t[{:.4}s]", t.elapsed().as_secs_f32());
    sum
}

fn p2(input: &str) -> usize {
    let factories: Vec<_> = input.lines().take(3).map(|s| Factory::from_str(s).unwrap()).collect();
    let mut sum = 1;
    let t = Instant::now();
    for (i, &factory) in factories.iter().enumerate() {
        println!("{}/{} Complete\t[{:.4}s]", i, factories.len(), t.elapsed().as_secs_f32());
        sum *= run(factory, 32).1;
    }
    println!("All Complete\t[{:.4}s]", t.elapsed().as_secs_f32());
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
}

impl Factory {
    pub fn new(id: u8, costs: [(u8, u8); 4]) -> Self {
        Factory { id, costs, mats: [0; 4], bots: [1, 0, 0, 0], building: None }
    }

    pub fn build_ore(&mut self) {
        self.mats[Mat::Ore as usize] -= self.costs[Mat::Ore as usize].0 as usize;
        self.building = Some(Mat::Ore as u8);
    }

    pub fn build_clay(&mut self) {
        self.mats[Mat::Ore as usize] -= self.costs[Mat::Clay as usize].0 as usize;
        self.building = Some(Mat::Clay as u8);
    }

    pub fn build_obsidian(&mut self) {
        self.mats[Mat::Ore as usize] -= self.costs[Mat::Obsidian as usize].0 as usize;
        self.mats[Mat::Clay as usize] -= self.costs[Mat::Obsidian as usize].1 as usize;
        self.building = Some(Mat::Obsidian as u8);
    }

    pub fn build_geode(&mut self) {
        self.mats[Mat::Ore as usize] -= self.costs[Mat::Geode as usize].0 as usize;
        self.mats[Mat::Obsidian as usize] -= self.costs[Mat::Geode as usize].1 as usize;
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


fn run(factory: Factory, max_time: usize) -> (usize, usize) {
    let mut max_quality = 0;
    let mut max_geodes = 0;
    let mut fs = VecDeque::new();
    // fs.push_back((factory, vec![], max_time));
    fs.push_back((factory, max_time));

    let max_possible_additions = sum_up(max_time);

    while !fs.is_empty() {
        let (mut f, time) = fs.pop_front().unwrap();
        let time = time - 1;
        f.harvest();

        if f.quality() > max_quality {
            max_geodes = max_geodes.max(f.mats[Mat::Geode as usize]);
            max_quality = max_quality.max(f.quality());
        }
        if time == 0 { continue; }


        // Decision Tree Pruning:
        if f.quality() < (max_quality as f32 * 0.75) as usize {  continue;  }

        if f.mats[Mat::Geode as usize] < max_geodes.saturating_sub(2) { continue; }
        if f.bots[Mat::Geode as usize] > 0 && time > max_time / 2 { continue; }
        if f.mats[Mat::Geode as usize] + max_possible_additions[time] <= max_geodes { continue; }

        if  f.mats[Mat::Ore as usize] >= f.costs[Mat::Geode as usize].0 as usize 
            && f.mats[Mat::Obsidian as usize] >= f.costs[Mat::Geode as usize].1 as usize 
        {
            let mut nf = f;
            nf.build_robot(Mat::Geode);
            fs.push_back((nf, time));
        } else {
            let max_ore = f.costs.iter().map(|c| c.0).max().unwrap() as usize + 1;
            let max_clay = f.costs[2].1 as usize + 1;
            let max_obs = f.costs[3].1 as usize + 1;

            for m in [Mat::Obsidian, Mat::Clay, Mat::Ore] {
                let (cost_a, cost_b) = f.costs[m as usize];
    
                let affordable = match m {
                    Mat::Ore =>      f.mats[Mat::Ore as usize] <= max_ore 
                                     && f.mats[Mat::Ore as usize] >= cost_a as usize,
                    Mat::Clay =>     f.mats[Mat::Ore as usize] >= cost_a as usize
                                     && f.mats[Mat::Clay as usize] <= max_clay,
                    Mat::Obsidian => f.mats[Mat::Ore as usize] >= cost_a as usize
                                     && f.mats[Mat::Clay as usize] >= cost_b as usize
                                     && f.mats[Mat::Obsidian as usize] <= max_obs,
                    Mat::Geode =>    f.mats[Mat::Ore as usize] >= cost_a as usize 
                                     && f.mats[Mat::Obsidian as usize] >= cost_b as usize,
                };
                if affordable {
                    let mut nf = f;
                    nf.build_robot(m);
                    fs.push_back((nf, time));
                }
            }
            fs.push_back((f, time));
        }
    }
    dbg!(max_quality, max_geodes);
    (max_quality, max_geodes)
}