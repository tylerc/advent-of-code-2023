use std::collections::HashMap;

use crate::fnv1::BuildFnv1Hasher;

type BrickId = usize;

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

    fn update_max(&mut self, other: &Pos) {
        if other.x > self.x {
            self.x = other.x;
        }
        if other.y > self.y {
            self.y = other.y;
        }
        if other.z > self.z {
            self.z = other.z;
        }
    }
}

#[derive(Clone)]
struct World {
    pos_to_id: Vec<Option<BrickId>>,
    id_to_pos: Vec<Vec<Pos>>,
    max_brick_id: BrickId,
    height: i64,
    width: i64,
    depth: i64,
}

impl World {
    fn new(input: &str) -> World {
        let mut id: BrickId = 0;
        let mut pos_to_id: HashMap<Pos, BrickId, BuildFnv1Hasher> =
            HashMap::with_hasher(BuildFnv1Hasher);
        let mut id_to_pos: Vec<Vec<Pos>> = Vec::new();
        let mut max = Pos { x: 0, y: 0, z: 0 };

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
            max.update_max(&start);
            max.update_max(&end);

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

            id_to_pos.push(pos_list);

            id += 1;
        }

        max.x += 1;
        max.y += 1;
        max.z += 1;
        let mut pos_to_id_vec = vec![None; (max.x * max.y * max.z) as usize];
        for (pos, id) in pos_to_id {
            pos_to_id_vec[World::pos_to_index_explicit(max.x, max.y, &pos)] = Some(id);
        }

        World {
            pos_to_id: pos_to_id_vec,
            id_to_pos,
            max_brick_id: id - 1,
            width: max.x,
            height: max.y,
            depth: max.z,
        }
    }

    fn pos_to_index(&self, pos: &Pos) -> usize {
        (pos.x + self.height * (pos.y + self.width * pos.z)) as usize
    }

    fn pos_to_index_explicit(width: i64, height: i64, pos: &Pos) -> usize {
        (pos.x + height * (pos.y + width * pos.z)) as usize
    }

    fn can_fall(&self, brick_id: BrickId) -> bool {
        let mut saw_brick_id = false;

        for pos in self.id_to_pos[brick_id].iter() {
            saw_brick_id = true;

            let pos_below = Pos {
                x: pos.x,
                y: pos.y,
                z: pos.z - 1,
            };
            if pos_below.z <= 0 {
                return false;
            }
            if let Some(below_brick_id) = self.pos_to_id[self.pos_to_index(&pos_below)] {
                if below_brick_id != brick_id {
                    return false;
                }
            }
        }

        saw_brick_id
    }

    fn fall(&mut self, brick_id: BrickId) {
        let positions = &mut self.id_to_pos[brick_id];

        for pos in positions.iter() {
            self.pos_to_id[World::pos_to_index_explicit(self.width, self.height, pos)] = None;
        }
        for pos in positions.iter_mut() {
            let new_pos = Pos {
                x: pos.x,
                y: pos.y,
                z: pos.z - 1,
            };
            self.pos_to_id[World::pos_to_index_explicit(self.width, self.height, &new_pos)] =
                Some(brick_id);
            *pos = new_pos;
        }
    }

    fn fall_until_settled(&mut self) {
        loop {
            let mut moved = false;
            for id in 0..=self.max_brick_id {
                while self.can_fall(id) {
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
        let old_positions = std::mem::take(&mut self.id_to_pos[brick_id]);
        for pos in old_positions {
            let index = self.pos_to_index(&pos);
            self.pos_to_id[index] = None;
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
        'outer: for (id, pos_list) in alternate_reality.id_to_pos.iter().enumerate() {
            for pos in pos_list {
                if let Some(orig_brick_id) = self.pos_to_id[self.pos_to_index(pos)] {
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
        let min = Pos { x: 0, y: 0, z: 0 };
        let max = Pos {
            x: self.width - 1,
            y: self.height - 1,
            z: self.depth - 1,
        };

        println!("min: {:?} max: {:?}", min, max);

        for x in min.x..=max.x {
            print!("{}", x);
        }
        println!();
        for z in (min.z..=max.z).rev() {
            for x in min.x..=max.x {
                let mut found: Vec<BrickId> = Vec::new();
                for y in min.y..=max.y {
                    if let Some(brick_id) = self.pos_to_id[self.pos_to_index(&Pos { x, y, z })] {
                        found.push(brick_id);
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
                    if let Some(brick_id) = self.pos_to_id[self.pos_to_index(&Pos { x, y, z })] {
                        found.push(brick_id);
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
