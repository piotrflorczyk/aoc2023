use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Clone, Debug)]
struct Module {
    name: String,
    typee: ModuleType,
    destinations: Vec<String>,
}

#[derive(Clone, Debug)]
struct Signal {
    src: String,
    dst: String,
    sig: bool,
}

#[derive(Clone, Debug)]
enum ModuleState {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Empty,
}

impl From<&str> for ModuleType {
    fn from(value: &str) -> Self {
        match value.chars().next().unwrap() {
            '&' => Self::Conjunction,
            '%' => Self::FlipFlop,
            'b' => Self::Broadcaster,
            _ => unreachable!(),
        }
    }
}

impl Module {
    fn create_init_state(&self) -> ModuleState {
        match self.typee {
            ModuleType::Conjunction => ModuleState::Conjunction(HashMap::new()),
            ModuleType::FlipFlop => ModuleState::FlipFlop(false),
            _ => ModuleState::Empty,
        }
    }

    fn process_signal(&self, sig: &Signal, module_state: &mut ModuleState) -> Vec<Signal> {
        match self.typee {
            ModuleType::Broadcaster => self
                .destinations
                .iter()
                .map(|d| Signal {
                    src: self.name.clone(),
                    dst: d.clone(),
                    sig: sig.sig,
                })
                .collect(),
            ModuleType::FlipFlop if !sig.sig => {
                if let ModuleState::FlipFlop(state) = module_state {
                    *state = !(*state);
                    self.destinations
                        .iter()
                        .map(|d| Signal {
                            src: self.name.clone(),
                            dst: d.clone(),
                            sig: *state,
                        })
                        .collect()
                } else {
                    unreachable!()
                }
            }
            ModuleType::Conjunction => {
                if let ModuleState::Conjunction(state) = module_state {
                    state.insert(sig.src.clone(), sig.sig);
                    let sig_val = state.values().any(|v| !v);
                    self.destinations
                        .iter()
                        .map(|d| Signal {
                            src: self.name.clone(),
                            dst: d.clone(),
                            sig: sig_val,
                        })
                        .collect()
                } else {
                    unreachable!()
                }
            }
            _ => vec![],
        }
    }
}

impl From<&str> for Module {
    fn from(value: &str) -> Self {
        let splitted = value.split_once(" -> ").unwrap();
        let typee = ModuleType::from(splitted.0);
        let name = if typee == ModuleType::Broadcaster {
            "broadcaster".to_string()
        } else {
            splitted.0[1..].to_string()
        };
        Self {
            name,
            typee,
            destinations: splitted
                .1
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        }
    }
}

fn simulate_button_press(
    modules_map: &HashMap<String, Module>,
    modules_states: &mut HashMap<String, ModuleState>,
    mut cb: impl FnMut(&Signal),
) {
    let mut queue = VecDeque::from([Signal {
        src: "button".to_string(),
        dst: "broadcaster".to_string(),
        sig: false,
    }]);
    while let Some(signal) = queue.pop_front() {
        cb(&signal);
        if modules_map.contains_key(&signal.dst) {
            let module_state = modules_states.get_mut(&signal.dst).unwrap();
            let new_signals = modules_map[&signal.dst].process_signal(&signal, module_state);
            queue.extend(new_signals.into_iter());
        }
    }
}

fn load_input() -> (HashMap<String, Module>, HashMap<String, ModuleState>) {
    let modules_map = include_str!("../../input/day20")
        .lines()
        .map(Module::from)
        .map(|m| (m.name.clone(), m))
        .collect::<HashMap<_, _>>();

    let mut modules_states: HashMap<String, ModuleState> = modules_map
        .values()
        .map(|m| (m.name.clone(), m.create_init_state()))
        .collect();

    // connect conjunction inputs
    modules_map.values().for_each(|m| {
        m.destinations.iter().for_each(|d| {
            if modules_map.contains_key(d) && modules_map[d].typee == ModuleType::Conjunction {
                if let Some(ModuleState::Conjunction(map)) = modules_states.get_mut(d) {
                    map.insert(m.name.clone(), false);
                }
            }
        });
    });
    (modules_map, modules_states)
}

fn p1() {
    let (modules_map, mut modules_states) = load_input();
    let mut signal_counts = vec![0; 2];
    (0..1000).for_each(|_| {
        simulate_button_press(&modules_map, &mut modules_states, |sig| {
            signal_counts[sig.sig as usize] += 1;
        });
    });

    let p1 = signal_counts.iter().product::<usize>();
    println!("p1: {p1}");
}

fn p2() {
    let (modules_map, mut modules_states) = load_input();
    let mut interesting_signals: Vec<u64> = vec![];
    let mut presses = 0;
    while interesting_signals.len() != 4 {
        presses += 1;
        simulate_button_press(&modules_map, &mut modules_states, |signal| {
            if signal.dst == "hf" && signal.sig {
                interesting_signals.push(presses);
            }
        });
    }
    let p2 = interesting_signals.iter().product::<u64>();
    println!("p2: {p2}");
}

fn main() {
    p1();
    p2();
}
