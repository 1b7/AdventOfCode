use std::collections::{HashMap, VecDeque};

fn main() {
    let mut modules = HashMap::new();
    for line in include_str!("../../../input/20").lines() {
        let (left, right) = line.split_once(" -> ").unwrap();

            let dests: Vec<&str> = right.split(", ").collect();

            let (name, module): (String, Module);
            if left == "broadcaster" {
                name = left.to_string();
                module = Module::new_broadcaster(name.clone(), dests);
            } else if left.starts_with("%") {
                name = left.replace("%", "");
                module = Module::new_flip_flop(name.clone(), dests);
            } else {
                name = left.replace("&", "");
                module = Module::new_conjunction(name.clone(), dests);
            };
            modules.insert(name, module);
    }

    // Update inputs for conjunctions
    let names = modules.clone().into_keys().collect::<Vec<_>>();
    for i in 0..names.len() {
        let src = modules.get(&names[i]).unwrap().clone();
        let outputs = &src.outputs;

        for j in 0..names.len() {
            if i == j { continue }
            let oth = modules.get_mut(&names[j]).unwrap();
            if oth.mtype == ModuleType::Conjunction {
                if outputs.contains(&names[j].as_str()) {
                    oth.input_states.insert(src.name.to_string(), false);
                }
            }
        }
    }

    println!("Part 1: {}", p1(&mut modules)); 
    // NOTE: Part 2 was done via analysis of the graph of connected modules, with pen and paper.

    /* EXPLANATION
       It turns out that there are effectively 4 binary counters some of which's 
       bits are connected to a conjunction.

       The sum 2^n over each of these bits, where n is the significance of the bit,
       gives us the point at which that counter's Conjunction outputs a high signal.

       So if we find this value for all 4 counters, and find the lowest-common-multiple
       of those four numbers, we get our answer for Part 2.

       fmt.py was used as a utility script to format the input appropriately as
       input to GraphViz.
    */
}

fn p1(modules: &mut HashMap<String, Module> ) -> usize {
    let (mut high, mut low) = (0, 0);
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back(("button", "broadcaster", false));

        while pulses.len() > 0 {
            let (from, to, pulse) = pulses.pop_front().unwrap();
            if pulse { high += 1 } else { low += 1}

            // Skip untyped modules:
            if !modules.contains_key(to) { continue; }

            let m = modules.get_mut(to).unwrap();
            if let Some(new_pulses) = m.receive(pulse, from.to_string()) {
                for target in new_pulses.1 {
                    pulses.push_back((to, target, new_pulses.0))
                }
            }
        }
    }
    high * low
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleType { Broadcaster, Conjunction, FlipFlop }

#[derive(Debug, Clone)]
struct Module {
    name: String,
    outputs: Vec<&'static str>,
    pulse_state: bool,
    input_states: HashMap<String, bool>,
    mtype: ModuleType
}

impl Module {
    pub fn new_broadcaster(name: String, outputs: Vec<&'static str>) -> Self {
        Self { name, outputs, pulse_state: false, input_states: HashMap::new(), mtype: ModuleType::Broadcaster }
    }

    pub fn new_flip_flop(name: String, outputs: Vec<&'static str>) -> Self {
        Self { name, outputs, pulse_state: false, input_states: HashMap::new(), mtype: ModuleType::FlipFlop }
    }

    pub fn new_conjunction(name: String, outputs: Vec<&'static str>) -> Self {
        Self { name, outputs, pulse_state: false, input_states: HashMap::new(), mtype: ModuleType::Conjunction }
    }

    pub fn receive(&mut self, pulse: bool, from: String) -> Option<(bool, &[&'static str])> {
        match self.mtype {
            ModuleType::Broadcaster => { Some((pulse, &self.outputs))  },
            ModuleType::FlipFlop => {
                if !pulse {
                    self.pulse_state = !self.pulse_state;
                    Some((self.pulse_state, &self.outputs))
                } else {
                    None
                }
            },
            ModuleType::Conjunction => {
                self.input_states.insert(from, pulse);
                if self.input_states.iter().all(|(_f, &p)| p) {
                    Some((false, &self.outputs))
                } else {
                    Some((true, &self.outputs))
                }
            }
        }
    }
}