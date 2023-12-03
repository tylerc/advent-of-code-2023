use std::collections::{HashMap, HashSet};

type Coord = (i32, i32);
type PartNumValue = i32;

#[derive(Debug, Copy, Clone)]
enum SchematicItem {
    Blank,
    PartNumber { num: PartNumValue, len: i32 },
    Symbol(char),
}

#[derive(Debug)]
struct Schematic {
    map: HashMap<Coord, SchematicItem>,
}

impl Schematic {
    fn new() -> Schematic {
        Schematic {
            map: HashMap::new(),
        }
    }

    fn record(&mut self, coord: Coord, item: SchematicItem) {
        self.map.insert(coord, item);
    }

    fn has_adjacent_symbol(&self, coord: Coord) -> bool {
        let offsets: [Coord; 8] = [
            (0, -1),  // Top
            (1, -1),  // Top-right
            (1, 0),   // Right
            (1, 1),   // Bottom-right
            (0, 1),   // Bottom
            (-1, 1),  // Bottom-left
            (-1, 0),  // Left
            (-1, -1), // Top-left
        ];

        for offset in offsets {
            let offset_coord: Coord = (coord.0 + offset.0, coord.1 + offset.1);
            match self.map.get(&offset_coord) {
                Some(SchematicItem::Symbol(_)) => return true,
                _ => continue,
            }
        }

        false
    }

    fn part_numbers_fill_map(&mut self) {
        let mut map_filled = self.map.clone();
        for (coord, item) in self.map.iter() {
            if let SchematicItem::PartNumber { len, .. } = item {
                for i in 1..*len {
                    let coord_new: Coord = (coord.0 + i, coord.1);
                    map_filled.insert(coord_new, *item);
                }
            }
        }
        self.map = map_filled;
    }

    fn adjacent_part_numbers(&self, coord: Coord) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let offsets: [Coord; 8] = [
            (0, -1),  // Top
            (1, -1),  // Top-right
            (1, 0),   // Right
            (1, 1),   // Bottom-right
            (0, 1),   // Bottom
            (-1, 1),  // Bottom-left
            (-1, 0),  // Left
            (-1, -1), // Top-left
        ];

        let mut numbers_seen: HashSet<PartNumValue> = HashSet::new();
        for offset in offsets {
            let coord_checking = (coord.0 + offset.0, coord.1 + offset.1);
            if let Some(SchematicItem::PartNumber { num, .. }) = self.map.get(&coord_checking) {
                if numbers_seen.get(num).is_none() {
                    result.push(*num);
                    numbers_seen.insert(*num);
                }
            }
        }

        result
    }

    fn is_valid_part(&self, coord: Coord) -> Option<PartNumValue> {
        match self.map.get(&coord) {
            Some(SchematicItem::PartNumber { num, len }) => {
                for i in 0..*len {
                    let coord_checking: Coord = (coord.0 + i, coord.1);
                    if self.has_adjacent_symbol(coord_checking) {
                        return Some(*num);
                    }
                }

                None
            }
            _ => None,
        }
    }

    fn sum_valid_parts(&self) -> i32 {
        let mut sum: i32 = 0;

        for coord in self.map.keys() {
            if let Some(value) = self.is_valid_part(*coord) {
                sum += value;
            }
        }

        sum
    }

    fn gear_ratio(&self, coord: Coord) -> i32 {
        match self.map.get(&coord) {
            Some(SchematicItem::Symbol('*')) => {
                let adjacent = self.adjacent_part_numbers(coord);
                if adjacent.len() == 2 {
                    adjacent.into_iter().product()
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn sum_gear_ratios(&mut self) -> i32 {
        self.part_numbers_fill_map();
        self.map.keys().map(|coord| self.gear_ratio(*coord)).sum()
    }

    fn parse(input: &str) -> Schematic {
        let mut schematic = Schematic::new();

        for (y, line) in input.split('\n').enumerate() {
            let mut number_building: Option<(Coord, String)> = None;
            let push_number =
                |schematic: &mut Schematic, number_building: &mut Option<(Coord, String)>| {
                    if let Some((coord, part_str)) = number_building {
                        let num: PartNumValue = part_str
                            .parse()
                            .expect("Part number string should be a valid number.");
                        schematic.record(
                            *coord,
                            SchematicItem::PartNumber {
                                num,
                                len: part_str.len() as i32,
                            },
                        );
                        *number_building = None;
                    }
                };

            for (x, char) in line.chars().enumerate() {
                let coord: Coord = (x as i32, y as i32);

                if char == '.' {
                    schematic.record(coord, SchematicItem::Blank);
                    push_number(&mut schematic, &mut number_building);
                } else if char.is_ascii_digit() {
                    match &mut number_building {
                        None => {
                            let mut str = String::new();
                            str.push(char);
                            number_building = Some((coord, str));
                        }
                        Some((_, str)) => {
                            str.push(char);
                        }
                    }
                } else {
                    schematic.record(coord, SchematicItem::Symbol(char));
                    push_number(&mut schematic, &mut number_building);
                }
            }

            push_number(&mut schematic, &mut number_building);
        }

        schematic
    }
}

pub fn day03_part_1(input: &str) -> i32 {
    Schematic::parse(input).sum_valid_parts()
}

pub fn day03_part_2(input: &str) -> i32 {
    Schematic::parse(input).sum_gear_ratios()
}

#[cfg(test)]
mod tests {
    use crate::day03::{day03_part_1, day03_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day03_part_1(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            4361
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day03_part_2(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            467835
        );
    }
}
