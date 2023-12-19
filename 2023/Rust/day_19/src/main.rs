use std::{collections::HashMap, time::Instant};

use regex::Regex;

fn main() {
    let s = include_str!("../../../input/19");
    let (workflows, parts) = parse_input(s);

    let t = Instant::now();
    let p1 = p1(&workflows, &parts);
    let e = t.elapsed().as_micros();
    println!("{p1} ({e}us)");

    let t = Instant::now();
    let p2 = p2(&workflows);
    let e = t.elapsed().as_micros();
    println!("{p2} ({e}us)");
}

fn p2(wfs: &HashMap<&str, Vec<(&str, Cmp, &str)>>) -> usize {
    let mut ranges: HashMap<&str, PartRange> = HashMap::new();
    ranges.insert("in", PartRange { x: (1, 4000), m: (1, 4000), a: (1, 4000), s: (1, 4000) });
    let mut locs = vec!["in"];

    let mut accepting_ranges = vec![];

    while locs.len() > 0 {
        let loc = locs.pop().unwrap();
        if loc == "R" || loc == "A" { continue; }
        let ruleset = wfs.get(loc).unwrap();
        let mut range = ranges.get(loc).unwrap().clone();
        for &(metric, cmp, dest) in ruleset {
            let mut new_range = range.clone();
            match metric {
                "x" => match cmp {
                    Cmp::Gt(n) => { new_range.x.0 = n + 1; range.x.1 = n },
                    Cmp::Lt(n) => { new_range.x.1 = n - 1; range.x.0 = n },
                    Cmp::Pass  => {}
                },
                "m" => match cmp {
                    Cmp::Gt(n) => { new_range.m.0 = n + 1; range.m.1 = n },
                    Cmp::Lt(n) => { new_range.m.1 = n - 1; range.m.0 = n },
                    Cmp::Pass  => {}
                },
                "a" => match cmp {
                    Cmp::Gt(n) => { new_range.a.0 = n + 1; range.a.1 = n },
                    Cmp::Lt(n) => { new_range.a.1 = n - 1; range.a.0 = n },
                    Cmp::Pass  => {}
                },
                "s" => match cmp {
                    Cmp::Gt(n) => { new_range.s.0 = n + 1; range.s.1 = n },
                    Cmp::Lt(n) => { new_range.s.1 = n - 1; range.s.0 = n },
                    Cmp::Pass  => {}
                },
                _ => panic!()
            };
            if dest == "A" { 
                accepting_ranges.push(new_range); 
            } else if dest != "R" { 
                ranges.insert(dest, new_range);
            }
            locs.push(dest);
        }
    }
    accepting_ranges.iter().map(|x| x.possibilities()).sum()
}

fn p1(wfs: &HashMap<&str, Vec<(&str, Cmp, &str)>>, pts: &[Part]) -> usize {
    let process_part = |part: &Part| -> usize {
        let mut wf = "in";
        loop {
            let ruleset = wfs.get(wf).unwrap();

            for rule in ruleset {
                let metric = match rule.0 {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    _   => panic!("Unrecognised Metric Requested")
                };

                if rule.1 == Cmp::Lt(0) || rule.1.test(metric) {
                    if rule.2 == "A" { return part.sum() } else if rule.2 == "R" { return 0 }
                    wf = rule.2;
                    break;
                }
            }
        }
    };
    pts.iter().map(|p| process_part(p)).sum()
}

fn parse_input(s: &str) -> (HashMap<&str, Vec<(&str, Cmp, &str)>>, Vec<Part>) {
    fn parse_rule(rule: &str) -> (&str, Cmp, &str) {
        if !rule.contains(":") { return ("x", Cmp::Pass, rule) }
    
        let (condition, target) = rule.split_once(':').unwrap();
        let (metric, n) = condition.split_once(['<', '>']).unwrap();
        
        let comparison = if condition.contains('<') {
            Cmp::Lt(n.parse().unwrap())
        } else {
            Cmp::Gt(n.parse().unwrap())
        };
    
        (metric, comparison, target)
    }

    let (workflows, parts) = s
    .split_once("\n\n")
    .unwrap();

    let wf_re = Regex::new(r"[\w\d<>:]+").unwrap();
    let workflows = workflows.lines()
        .map(|line| {
            let mut matches = wf_re.find_iter(line);
            let name = matches.next().unwrap().as_str();
            let rules: Vec<_> = matches.map(|m| parse_rule(m.as_str())).collect();

            (name, rules)
        })
        .collect::<HashMap<&str, _>>();

    let part_re = Regex::new(r"\d+").unwrap();
    let parts = parts.lines()
        .map(|line| {
            let mut ns = part_re.find_iter(line)
                .map(|n| n.as_str().parse::<usize>().unwrap());

            Part {
                x: ns.next().unwrap(),
                m: ns.next().unwrap(),
                a: ns.next().unwrap(),
                s: ns.next().unwrap()
            }
        })
        .collect::<Vec<_>>();

    (workflows, parts)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cmp { Gt(usize), Lt(usize), Pass }
impl Cmp {
    pub fn test(&self, x: usize) -> bool {
        match self {
            Cmp::Gt(n) => x > *n,
            Cmp::Lt(n) => x < *n,
            Cmp::Pass => true
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part { x: usize, m: usize, a: usize, s: usize }
impl Part { 
    pub fn sum(&self) -> usize { self.x + self.m + self.a + self.s }
}

#[derive(Debug, Clone, Copy)]
struct PartRange { x: (usize, usize), m: (usize, usize), a: (usize, usize), s: (usize, usize) }
impl PartRange { 
    pub fn possibilities(&self) -> usize { 
        (1 + self.x.1 - self.x.0) * (1 + self.m.1 - self.m.0) * (1 + self.a.1 - self.a.0) * (1 + self.s.1 - self.s.0)
    }
}
