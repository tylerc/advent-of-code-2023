use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleState {
    On,
    Off,
}
use ModuleState::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseType {
    Low,
    High,
}
use PulseType::*;

#[derive(Debug)]
enum ModuleType<'a> {
    Button,
    Broadcaster,
    FlipFlop(ModuleState),
    Conjunction(HashMap<&'a str, PulseType>, bool),
}
use ModuleType::*;

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    kind: ModuleType<'a>,
    destinations: Vec<&'a str>,
}

struct QueueMessage<'a> {
    source: &'a str,
    destination: &'a str,
    pulse: PulseType,
}

struct Queue<'a> {
    queue: VecDeque<QueueMessage<'a>>,
    high_count: i64,
    low_count: i64,
    rx_received_low_pulse: bool,
}

impl<'a> Queue<'a> {
    fn new() -> Queue<'a> {
        Queue {
            queue: VecDeque::new(),
            high_count: 0,
            low_count: 0,
            rx_received_low_pulse: false,
        }
    }

    fn push_back(&mut self, msg: QueueMessage<'a>) {
        match msg.pulse {
            High => self.high_count += 1,
            Low => {
                self.low_count += 1;
                if msg.destination == "rx" {
                    self.rx_received_low_pulse = true;
                }
            }
        }

        self.queue.push_back(msg);
    }

    fn pop_front(&mut self) -> Option<QueueMessage<'a>> {
        self.queue.pop_front()
    }

    fn score(&self) -> i64 {
        self.high_count * self.low_count
    }
}

impl<'a> Module<'a> {
    fn send(&self, pulse: PulseType, queue: &mut Queue<'a>) {
        for destination in self.destinations.iter() {
            queue.push_back(QueueMessage {
                source: self.name,
                destination,
                pulse,
            });
        }
    }

    fn pulse_process(&mut self, message: QueueMessage<'a>, queue: &mut Queue<'a>) {
        match &mut self.kind {
            Button => self.send(Low, queue),
            Broadcaster => {
                self.send(message.pulse, queue);
            }
            FlipFlop(state) => {
                if message.pulse == Low {
                    let (new_state, pulse_sending) =
                        if *state == On { (Off, Low) } else { (On, High) };
                    *state = new_state;
                    self.send(pulse_sending, queue);
                }
            }
            Conjunction(inputs, all_high) => {
                inputs.insert(message.source, message.pulse);
                if inputs.values().all(|input| *input == High) {
                    *all_high = true;
                    self.send(Low, queue);
                } else {
                    self.send(High, queue);
                }
            }
        }
    }
}

type ModuleMap<'a> = HashMap<&'a str, Module<'a>>;

fn day20_parse(input: &str) -> (ModuleMap, Module) {
    let mut modules: ModuleMap = HashMap::new();
    let mut conjunctions: Vec<&str> = Vec::new();

    for line in input.split('\n') {
        let mut halves = line.split(" -> ");
        let type_and_name = halves
            .next()
            .expect("Expected to find type and name string.");
        let destinations: Vec<&str> = halves
            .next()
            .expect("Expected to find destinations.")
            .split(", ")
            .collect();

        if type_and_name == "broadcaster" {
            modules.insert(
                type_and_name,
                Module {
                    name: type_and_name,
                    kind: Broadcaster,
                    destinations,
                },
            );
        } else if &type_and_name[0..1] == "%" {
            modules.insert(
                &type_and_name[1..],
                Module {
                    name: &type_and_name[1..],
                    kind: FlipFlop(Off),
                    destinations,
                },
            );
        } else if &type_and_name[0..1] == "&" {
            modules.insert(
                &type_and_name[1..],
                Module {
                    name: &type_and_name[1..],
                    kind: Conjunction(HashMap::new(), false),
                    destinations,
                },
            );
            conjunctions.push(&type_and_name[1..]);
        } else {
            unreachable!("Saw unexpected name: {}", type_and_name);
        }
    }

    for conj_name in conjunctions {
        let mut inputs: HashMap<&str, PulseType> = HashMap::new();

        for (module_name, Module { destinations, .. }) in modules.iter() {
            for dest in destinations.iter() {
                if &conj_name == dest {
                    inputs.insert(module_name, Low);
                }
            }
        }

        if let Some(Module {
            kind: Conjunction(inner_inputs, _),
            ..
        }) = modules.get_mut(conj_name)
        {
            *inner_inputs = inputs;
        } else {
            unreachable!("Conjunction list should contain only conjunctions.");
        }
    }

    let button = Module {
        name: "__BUTTON__",
        kind: Button,
        destinations: vec!["broadcaster"],
    };

    (modules, button)
}

// From https://en.wikipedia.org/wiki/Euclidean_algorithm#Implementations
fn gcd(a: i64, b: i64) -> i64 {
    /*
    * function gcd(a, b)
        while b ≠ 0
            t := b
            b := a mod b
            a := t
        return a*/
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// From https://en.wikipedia.org/wiki/Least_common_multiple#Calculation
fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

fn lcm_many(values: &[i64]) -> i64 {
    let mut result = values[0];

    for other in values.iter().skip(1) {
        result = lcm(result, *other);
    }

    result
}

pub fn day20_part_1(input: &str) -> i64 {
    let (mut modules, mut button) = day20_parse(input);

    let mut queue = Queue::new();
    for _ in 0..1000 {
        button.pulse_process(
            QueueMessage {
                source: "__ME__",
                destination: "__BUTTON__",
                pulse: Low,
            },
            &mut queue,
        );

        while let Some(message) = queue.pop_front() {
            if let Some(module) = modules.get_mut(message.destination) {
                module.pulse_process(message, &mut queue);
            }
        }
    }

    queue.score()
}

pub fn day20_part_2(input: &str) -> i64 {
    let (mut modules, mut button) = day20_parse(input);

    let mut queue = Queue::new();
    let mut presses = 0;
    let mut qq: Option<i64> = None;
    let mut gj: Option<i64> = None;
    let mut bc: Option<i64> = None;
    let mut bx: Option<i64> = None;
    loop {
        presses += 1;
        button.pulse_process(
            QueueMessage {
                source: "__ME__",
                destination: "__BUTTON__",
                pulse: Low,
            },
            &mut queue,
        );

        while let Some(message) = queue.pop_front() {
            if let Some(module) = modules.get_mut(message.destination) {
                module.pulse_process(message, &mut queue);
            }
        }

        if qq.is_none() {
            if let Some(Module { kind: Conjunction(_, true), .. }) = modules.get_mut("qq") {
                qq = Some(presses);
            }
        }
        if gj.is_none() {
            if let Some(Module { kind: Conjunction(_, true), .. }) = modules.get_mut("gj") {
                gj = Some(presses);
            }
        }
        if bc.is_none() {
            if let Some(Module { kind: Conjunction(_, true), .. }) = modules.get_mut("bc") {
                bc = Some(presses);
            }
        }
        if bx.is_none() {
            if let Some(Module { kind: Conjunction(_, true), .. }) = modules.get_mut("bx") {
                bx = Some(presses);
            }
        }
        if let (Some(qq), Some(gj), Some(bc), Some(bx)) = (qq, gj, bc, bx) {
            return lcm_many(&[qq, gj, bc, bx]);
        }

        if queue.rx_received_low_pulse {
            break;
        } else if presses % 100000 == 0 {
            println!("{}", presses);
        }
    }

    presses
}

#[cfg(test)]
mod tests {
    use crate::day20::day20_part_1;

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day20_part_1(
                "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
            ),
            32000000
        );

        assert_eq!(
            day20_part_1(
                "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
            ),
            11687500
        );
    }
}
