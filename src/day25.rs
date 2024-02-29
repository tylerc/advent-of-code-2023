use std::collections::{HashMap, HashSet};
use crate::fnv1::BuildFnv1Hasher;

trait Connections {
    fn connect(&mut self, left: &str, right: &str);
    fn reachable(&self, item: &str) -> usize;
    fn cut(&mut self, left: &str, right: &str);
    fn farthest_can_travel(&self, item: &str) -> usize;
}

type ConnectionMap = HashMap<String, HashSet<String>>;

impl Connections for ConnectionMap {
    fn connect(&mut self, left: &str, right: &str) {
        self.entry(left.to_owned())
            .or_default()
            .insert(right.to_owned());
        self.entry(right.to_owned())
            .or_default()
            .insert(left.to_owned());
    }

    fn reachable(&self, item: &str) -> usize {
        let mut seen: HashSet<&str> = HashSet::new();
        seen.insert(item);
        let mut to_see: Vec<&str> = vec![item];
        let empty_set = HashSet::new();

        while let Some(item) = to_see.pop() {
            let connections = self.get(item).unwrap_or(&empty_set);
            for conn in connections.iter() {
                if !seen.contains(conn.as_str()) {
                    to_see.push(conn.as_str());
                    seen.insert(conn.as_str());
                }
            }
        }

        seen.len()
    }

    fn cut(&mut self, left: &str, right: &str) {
        self.get_mut(left)
            .expect("Left side of cut should exist.")
            .remove(right);
        self.get_mut(right)
            .expect("Right side of cut should exist.")
            .remove(left);
    }

    fn farthest_can_travel(&self, item: &str) -> usize {
        let mut costs: HashMap<&str, usize, BuildFnv1Hasher> = HashMap::with_hasher(BuildFnv1Hasher);
        costs.insert(item, 0);
        let mut to_visit: Vec<(&str, usize)> = vec![(item, 0)];
        let empty_set = HashSet::new();

        while let Some((item, current_cost)) = to_visit.pop() {
            let next_cost = current_cost + 1;
            for conn in self.get(item).unwrap_or(&empty_set) {
                let entry = costs.entry(conn.as_str()).or_insert(usize::MAX);
                if *entry > next_cost {
                    *entry = next_cost;
                    to_visit.push((conn, next_cost));
                }
            }
        }

        *costs.values().max().expect("There should be a maximum value.")
    }
}

pub fn day25_part_1(input: &str) -> usize {
    let mut connections: ConnectionMap = HashMap::new();
    let mut pairs: Vec<(String, String)> = Vec::new();
    for line in input.split('\n') {
        let split: Vec<&str> = line.split(": ").collect();
        let left = split[0];
        for right in split[1].split(' ') {
            connections.connect(left, right);
            pairs.push((left.to_owned(), right.to_owned()));
        }
    }

    // The nodes that can reach all other nodes in the fewest steps are in the "middle" of the graph
    // and therefore our most likely candidate for being part of a pair of wires we need to cut:
    let travel_distances: HashMap<String, usize> = connections.keys().map(|key| {
        (key.clone(), connections.farthest_can_travel(key))
    }).collect();

    // Now that we have those distances, we can sort the pairs so that the most likely candidates
    // are processed first, making our runtime much closer to O(n) than O(n^3):
    pairs.sort_by(|a, b| {
        let travel_a = travel_distances.get(a.0.as_str()).unwrap() + travel_distances.get(a.1.as_str()).unwrap();
        let travel_b = travel_distances.get(b.0.as_str()).unwrap() + travel_distances.get(b.1.as_str()).unwrap();
        travel_a.cmp(&travel_b)
    });

    for (index1, pair1) in pairs.iter().enumerate() {
        for (index2, pair2) in pairs.iter().skip(index1 + 1).enumerate() {
            for pair3 in pairs.iter().skip(index2 + 1) {
                connections.cut(&pair1.0, &pair1.1);
                connections.cut(&pair2.0, &pair2.1);
                connections.cut(&pair3.0, &pair3.1);
                let left_count = connections.reachable(&pair1.0);
                let right_count = connections.reachable(&pair1.1);
                if left_count != right_count {
                    return left_count * right_count;
                }

                connections.connect(&pair1.0, &pair1.1);
                connections.connect(&pair2.0, &pair2.1);
                connections.connect(&pair3.0, &pair3.1);
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::day25::day25_part_1;

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day25_part_1(
                "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr",
            ),
            54
        );
    }
}
