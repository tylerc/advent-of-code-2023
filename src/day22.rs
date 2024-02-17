use std::collections::HashMap;

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

#[derive(Clone)]
struct World {
    pos_to_id: HashMap<Pos, BrickId>,
    id_to_pos: HashMap<BrickId, Vec<Pos>>,
    max_brick_id: BrickId,
}

impl World {
    fn new(input: &str) -> World {
        let mut id: BrickId = 0;
        let mut pos_to_id: HashMap<Pos, BrickId> = HashMap::new();
        let mut id_to_pos: HashMap<BrickId, Vec<Pos>> = HashMap::new();

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
            let mut pos_list: Vec<Pos> = Vec::new();
            pos_to_id.insert(start, id);
            pos_list.push(start);
            while current != end {
                current = Pos {
                    x: current.x + step.x,
                    y: current.y + step.y,
                    z: current.z + step.z,
                };
                pos_to_id.insert(current, id);
                pos_list.push(current);
            }

            id_to_pos.insert(id, pos_list);

            id += 1;
        }

        World {
            pos_to_id,
            id_to_pos,
            max_brick_id: id - 1,
        }
    }

    fn can_fall(&self, brick_id: BrickId) -> bool {
        let mut saw_brick_id = false;

        for pos in self
            .id_to_pos
            .get(&brick_id)
            .expect("Expected to find list of positions to fall test.")
            .iter()
        {
            saw_brick_id = true;

            let pos_below = Pos {
                x: pos.x,
                y: pos.y,
                z: pos.z - 1,
            };
            if pos_below.z <= 0 {
                return false;
            }
            if let Some(below_brick_id) = self.pos_to_id.get(&pos_below) {
                if *below_brick_id != brick_id {
                    return false;
                }
            }
        }

        saw_brick_id
    }

    fn fall(&mut self, brick_id: BrickId) {
        let old_positions: Vec<Pos> = std::mem::take(
            self.id_to_pos
                .get_mut(&brick_id)
                .expect("Expected to find list of positions to fall (1)."),
        );
        let new_positions = self
            .id_to_pos
            .get_mut(&brick_id)
            .expect("Expected to find list of positions to fall (2).");

        for pos in old_positions.iter() {
            self.pos_to_id.remove(pos);
        }
        for pos in old_positions.iter() {
            let new_pos = Pos {
                x: pos.x,
                y: pos.y,
                z: pos.z - 1,
            };
            self.pos_to_id.insert(new_pos, brick_id);
            new_positions.push(new_pos);
        }
    }

    fn fall_until_settled(&mut self) {
        loop {
            let mut moved = false;
            for id in 0..=self.max_brick_id {
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

    fn remove_brick(&mut self, brick_id: BrickId) {
        let old_positions = std::mem::take(
            self.id_to_pos
                .get_mut(&brick_id)
                .expect("Expected to find bricks to remove."),
        );
        for pos in old_positions {
            self.pos_to_id.remove(&pos);
        }
    }

    fn can_disintigrate(&self, brick_id: BrickId) -> bool {
        let mut alternate_reality = self.clone();
        alternate_reality.remove_brick(brick_id);
        for iter_id in 0..=self.max_brick_id {
            if iter_id == brick_id {
                continue;
            }

            if alternate_reality.can_fall(iter_id) {
                return false;
            }
        }

        true
    }

    fn count_disintigrate(&self, brick_id: BrickId) -> i64 {
        let mut alternate_reality = self.clone();
        alternate_reality.remove_brick(brick_id);
        alternate_reality.fall_until_settled();

        let mut moved: i64 = 0;
        'outer: for (id, pos_list) in alternate_reality.id_to_pos.iter() {
            for pos in pos_list {
                if let Some(orig_brick_id) = self.pos_to_id.get(pos) {
                    if orig_brick_id != id {
                        moved += 1;
                        continue 'outer;
                    }
                } else {
                    moved += 1;
                    continue 'outer;
                }
            }
        }

        moved
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut min = Pos { x: 0, y: 0, z: 0 };
        let mut max = Pos { x: 0, y: 0, z: 0 };
        for pos in self.pos_to_id.keys() {
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
                    if let Some(brick_id) = self.pos_to_id.get(&Pos { x, y, z }) {
                        found.push(*brick_id);
                    }
                }

                match found.len() {
                    0 => print!("."),
                    1 => {
                        print!(
                            "{}",
                            char::from_u32('A' as u32 + found[0] as u32)
                                .expect("Char should be valid.")
                        );
                    }
                    _ => print!("?"),
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
                    if let Some(brick_id) = self.pos_to_id.get(&Pos { x, y, z }) {
                        found.push(*brick_id);
                    }
                }

                match found.len() {
                    0 => print!("."),
                    1 => {
                        print!(
                            "{}",
                            char::from_u32('A' as u32 + found[0] as u32)
                                .expect("Char should be valid.")
                        );
                    }
                    _ => print!("?"),
                }
            }
            println!(" {}", z);
        }
    }
}

pub fn day22_part_1(input: &str) -> i64 {
    let mut bricks = World::new(input);
    bricks.fall_until_settled();
    let mut result: i64 = 0;
    for id in 0..=bricks.max_brick_id {
        if bricks.can_disintigrate(id) {
            result += 1;
        }
    }
    result
}

pub fn day22_part_2(input: &str) -> i64 {
    let mut bricks = World::new(input);
    bricks.fall_until_settled();
    let mut result: i64 = 0;
    for id in 0..=bricks.max_brick_id {
        result += bricks.count_disintigrate(id);
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
