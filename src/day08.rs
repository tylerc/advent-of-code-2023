use std::collections::HashMap;

pub fn day08_part_1(input: &str) -> i64 {
    let mut lines = input.split('\n');
    let instructions = lines.next().expect("Instructions line should exist.");
    lines.next().expect("Discard next line.");

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines {
        let label_to_connection_str: Vec<_> = line.split(" = ").collect();
        let label = label_to_connection_str[0];
        let connections: Vec<_> = label_to_connection_str[1]
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .collect();
        map.insert(label, (connections[0], connections[1]));
    }

    let mut steps_taken = 0;
    let mut location = "AAA";
    for char in instructions.chars().cycle() {
        if location == "ZZZ" {
            break;
        }

        steps_taken += 1;
        let connections = map
            .get(location)
            .expect("Expected to find connections but didn't.");
        if char == 'L' {
            location = connections.0;
        } else if char == 'R' {
            location = connections.1;
        } else {
            panic!("Received unexpected instuction: {}", char);
        }
    }

    steps_taken
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

pub fn day08_part_2(input: &str) -> i64 {
    let mut lines = input.split('\n');
    let instructions = lines.next().expect("Instructions line should exist.");
    lines.next().expect("Discard next line.");

    let mut map: Vec<(usize, usize)> = Vec::new();
    let mut label_to_index_map: HashMap<&str, usize> = HashMap::new();
    let mut label_to_index = |map: &mut Vec<(usize, usize)>, label: &str| -> usize {
        match label_to_index_map.get(label) {
            Some(index) => *index,
            None => {
                let index = map.len();
                map.push((usize::MAX, usize::MAX));
                label_to_index_map.insert(label.to_owned().leak(), index);
                index
            }
        }
    };
    let mut locations: Vec<usize> = Vec::new();
    let mut ending_indexes: Vec<usize> = Vec::new();

    for line in lines {
        let label_to_connection_str: Vec<_> = line.split(" = ").collect();
        let label = label_to_connection_str[0];
        let connections: Vec<_> = label_to_connection_str[1]
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .collect();

        let label_index = label_to_index(&mut map, label);
        map[label_index] = (
            label_to_index(&mut map, connections[0]),
            label_to_index(&mut map, connections[1]),
        );
        if label.ends_with('A') {
            locations.push(label_index);
        } else if label.ends_with('Z') {
            ending_indexes.push(label_index);
        }
    }

    let mut steps_to_cycle = vec![0; locations.len()];

    let mut steps_taken = 0;
    'outer: for char in instructions.chars().cycle() {
        if locations.iter().all(|loc| ending_indexes.contains(loc)) {
            break;
        }

        steps_taken += 1;

        for (index, loc) in locations.iter_mut().enumerate() {
            let connections = map
                .get(*loc)
                .expect("Expected to find connections but didn't.");
            if char == 'L' {
                *loc = connections.0;
            } else if char == 'R' {
                *loc = connections.1;
            } else {
                panic!("Received unexpected instuction: {}", char);
            }

            if ending_indexes.contains(loc) && steps_to_cycle[index] == 0 {
                steps_to_cycle[index] = steps_taken;
                if steps_to_cycle.iter().all(|steps| *steps > 0) {
                    steps_taken = lcm_many(&steps_to_cycle);
                    break 'outer;
                }
            }
        }
    }

    steps_taken
}

#[cfg(test)]
mod tests {
    use crate::day08::{day08_part_1, day08_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day08_part_1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            2
        );

        assert_eq!(
            day08_part_1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day08_part_2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }
}
