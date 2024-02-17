use std::collections::{HashMap, HashSet};

type BrickId = u64;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn new(coords: &str) -> Pos {
        let coords: Vec<i64> = coords
            .split(',')
            .map(|num| {
                num.parse::<i64>()
                    .expect("Expected coordinates to be valid integers.")
            })
            .collect();

        Pos {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

fn str_to_cubes(input: &str) -> (BrickId, HashMap<Pos, BrickId>) {
    let mut id: BrickId = 0;
    let mut result: HashMap<Pos, BrickId> = HashMap::new();

    for line in input.split('\n') {
        let mut split = line.split('~');
        let start_str = split
            .next()
            .expect("Expected to find left-hand coordinate.");
        let end_str = split
            .next()
            .expect("Expected to find right-hand coordinate.");
        let start = Pos::new(start_str);
        let end = Pos::new(end_str);
        let mut current = start;
        let step = Pos {
            x: if end.x > start.x { 1 } else { 0 },
            y: if end.y > start.y { 1 } else { 0 },
            z: if end.z > start.z { 1 } else { 0 },
        };
        result.insert(start, id);
        while current != end {
            current = Pos {
                x: current.x + step.x,
                y: current.y + step.y,
                z: current.z + step.z,
            };
            result.insert(current, id);
        }

        id += 1;
    }

    (id - 1, result)
}

trait World {
    fn can_fall(&self, brick_id: BrickId) -> bool;
    fn fall(&mut self, brick_id: BrickId);
    fn fall_until_settled(&mut self, max_brick_id: BrickId);
    fn can_disintigrate(&self, brick_id: BrickId, max_brick_id: BrickId) -> bool;
    fn count_disintigrate(&self, brick_id: BrickId, max_brick_id: BrickId) -> i64;
    fn print(&self);
}

impl World for HashMap<Pos, BrickId> {
    fn can_fall(&self, brick_id: BrickId) -> bool {
        let mut saw_brick_id = false;

        for (pos, other_brick_id) in self.iter() {
            if *other_brick_id != brick_id {
                continue;
            }

            saw_brick_id = true;

            let pos_below = Pos {
                x: pos.x,
                y: pos.y,
                z: pos.z - 1,
            };
            if pos_below.z <= 0 {
                return false;
            }
            if let Some(below_brick_id) = self.get(&pos_below) {
                if *below_brick_id != brick_id {
                    return false;
                }
            }
        }

        saw_brick_id
    }

    fn fall(&mut self, brick_id: BrickId) {
        let old_positions: Vec<Pos> = self
            .iter()
            .filter(|(_, other_brick_id)| **other_brick_id == brick_id)
            .map(|(pos, _)| *pos)
            .collect();

        for pos in old_positions.iter() {
            self.remove(&pos);
        }
        for pos in old_positions.iter() {
            let new_pos = Pos {
                x: pos.x,
                y: pos.y,
                z: pos.z - 1,
            };
            self.insert(new_pos, brick_id);
        }
    }

    fn fall_until_settled(&mut self, max_brick_id: BrickId) {
        loop {
            let mut moved = false;
            for id in 0..=max_brick_id {
                if self.can_fall(id) {
                    self.fall(id);
                    moved = true;
                }
            }
            if !moved {
                break;
            }
        }
    }

    fn can_disintigrate(&self, brick_id: BrickId, max_brick_id: BrickId) -> bool {
        let mut alternate_reality = self.clone();
        alternate_reality.retain(|_, other_brick_id| brick_id != *other_brick_id);
        for iter_id in 0..=max_brick_id {
            if iter_id == brick_id {
                continue;
            }

            if alternate_reality.can_fall(iter_id) {
                return false;
            }
        }

        true
    }

    fn count_disintigrate(&self, brick_id: BrickId, max_brick_id: BrickId) -> i64 {
        let mut alternate_reality = self.clone();
        alternate_reality.retain(|_, other_brick_id| brick_id != *other_brick_id);
        alternate_reality.fall_until_settled(max_brick_id);

        let mut moved: HashSet<BrickId> = HashSet::new();
        for (pos, alternate_reality_brick_id) in alternate_reality.iter() {
            if let Some(orig_brick_id) = self.get(pos) {
                if orig_brick_id != alternate_reality_brick_id {
                    moved.insert(*alternate_reality_brick_id);
                }
            } else {
                moved.insert(*alternate_reality_brick_id);
            }
        }

        moved.len() as i64
    }

    fn print(&self) {
        let mut min = Pos { x: 0, y: 0, z: 0 };
        let mut max = Pos { x: 0, y: 0, z: 0 };
        for pos in self.keys() {
            if pos.x < min.x {
                min.x = pos.x;
            }
            if pos.x > max.x {
                max.x = pos.x;
            }
            if pos.y < min.y {
                min.y = pos.y;
            }
            if pos.y > max.y {
                max.y = pos.y;
            }
            if pos.z < min.z {
                min.z = pos.z;
            }
            if pos.z > max.z {
                max.z = pos.z;
            }
        }

        println!("min: {:?} max: {:?}", min, max);

        for x in min.x..=max.x {
            print!("{}", x);
        }
        println!();
        for z in (min.z..=max.z).rev() {
            for x in min.x..=max.x {
                let mut found: Vec<BrickId> = Vec::new();
                for y in min.y..=max.y {
                    if let Some(brick_id) = self.get(&Pos { x, y, z }) {
                        found.push(*brick_id);
                    }
                }

                if found.len() > 1 {
                    print!("?");
                } else if found.len() == 1 {
                    print!(
                        "{}",
                        char::from_u32('A' as u32 + found[0] as u32)
                            .expect("Char should be valid.")
                    );
                } else {
                    print!(".");
                }
            }
            println!(" {}", z);
        }

        println!();

        for y in min.y..=max.y {
            print!("{}", y);
        }
        println!();
        for z in (min.z..=max.z).rev() {
            for y in min.y..=max.y {
                let mut found: Vec<BrickId> = Vec::new();
                for x in min.x..=max.x {
                    if let Some(brick_id) = self.get(&Pos { x, y, z }) {
                        found.push(*brick_id);
                    }
                }

                if found.len() > 1 {
                    print!("?");
                } else if found.len() == 1 {
                    print!(
                        "{}",
                        char::from_u32('A' as u32 + found[0] as u32)
                            .expect("Char should be valid.")
                    );
                } else {
                    print!(".");
                }
            }
            println!(" {}", z);
        }
    }
}

pub fn day22_part_1(input: &str) -> i64 {
    let (max_id, mut bricks) = str_to_cubes(input);
    bricks.fall_until_settled(max_id);
    let mut result: i64 = 0;
    for id in 0..=max_id {
        if bricks.can_disintigrate(id, max_id) {
            result += 1;
        }
    }
    result
}

pub fn day22_part_2(input: &str) -> i64 {
    let (max_id, mut bricks) = str_to_cubes(input);
    bricks.fall_until_settled(max_id);
    let mut result: i64 = 0;
    for id in 0..=max_id {
        result += bricks.count_disintigrate(id, max_id);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::day22::day22_part_1;
    use crate::day22::day22_part_2;

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day22_part_1(
                "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
            ),
            5
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day22_part_2(
                "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
            ),
            7
        );
    }
}
