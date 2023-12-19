use std::{collections::HashSet, str::Split};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Pos {
    row: i64,
    col: i64,
}

impl Pos {
    fn add(&self, other: &Pos) -> Pos {
        Pos {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

trait GridOps {
    fn bounds(&self) -> (Pos, Pos);
    fn print(&self);
}

impl GridOps for HashSet<Pos> {
    fn bounds(&self) -> (Pos, Pos) {
        let mut min = Pos {
            row: i64::MAX,
            col: i64::MAX,
        };
        let mut max = Pos {
            row: i64::MIN,
            col: i64::MIN,
        };
        for pos in self.iter() {
            if pos.row < min.row {
                min.row = pos.row;
            }
            if pos.col < min.col {
                min.col = pos.col;
            }
            if pos.row > max.row {
                max.row = pos.row;
            }
            if pos.col > max.col {
                max.col = pos.col;
            }
        }

        (min, max)
    }

    fn print(&self) {
        let (min, max) = self.bounds();
        for row in min.row..=max.row {
            for col in min.col..=max.col {
                if self.contains(&Pos { row, col }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

struct Part1Iter<'a>(Split<'a, char>);
impl<'a> Iterator for Part1Iter<'a> {
    type Item = (char, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.0.next() {
            let mut split = line.split(' ');
            let direction = split
                .next()
                .expect("Expected direction.")
                .chars()
                .next()
                .expect("Expect single char for direction.");
            let distance: u8 = split
                .next()
                .expect("Expected distance.")
                .parse()
                .expect("Expected distance to be a valid number.");

            Some((direction, distance))
        } else {
            None
        }
    }
}

impl<'a> Part1Iter<'a> {
    fn new(input: &'a str) -> Part1Iter<'a> {
        Part1Iter(input.split('\n'))
    }
}

struct Part2Iter<'a>(Split<'a, char>);
impl<'a> Iterator for Part2Iter<'a> {
    type Item = (char, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.0.next() {
            let hex = line.split(' ').nth(2).expect("Expected to find hex codes.");

            let distance =
                i64::from_str_radix(&hex[2..7], 16).expect("Expected a valid hex number.");
            let direction = match &hex[7..8] {
                "0" => 'R',
                "1" => 'D',
                "2" => 'L',
                "3" => 'U',
                unknown => unreachable!("Unexpected direction hex number: {}", unknown),
            };

            Some((direction, distance))
        } else {
            None
        }
    }
}

impl<'a> Part2Iter<'a> {
    fn new(input: &'a str) -> Part2Iter<'a> {
        Part2Iter(input.split('\n'))
    }
}

fn shoelace(points: &[Pos]) -> i64 {
    let sum: i64 = points
        .iter()
        .zip(points.iter().skip(1))
        .map(|(p1, p2)| p1.col * p2.row - p1.row * p2.col)
        .sum();
    sum / 2
}

pub fn day18_part_1(input: &str) -> i64 {
    let mut pos = Pos { row: 0, col: 0 };
    let mut points: Vec<Pos> = Vec::new();
    let mut total_distance: i64 = 0;
    points.push(pos);

    for (direction, distance) in Part1Iter::new(input) {
        let direction = match direction {
            'U' => Pos { row: -1, col: 0 },
            'D' => Pos { row: 1, col: 0 },
            'L' => Pos { row: 0, col: -1 },
            'R' => Pos { row: 0, col: 1 },
            unknown => unreachable!("Unexpected direction: {}", unknown),
        };

        pos = pos.add(&Pos {
            row: direction.row * distance as i64,
            col: direction.col * distance as i64,
        });
        total_distance += distance as i64;
        points.push(pos);
    }

    shoelace(&points) + (total_distance / 2) + 1
}

pub fn day18_part_2(input: &str) -> i64 {
    let mut pos = Pos { row: 0, col: 0 };
    let mut points: Vec<Pos> = Vec::new();
    let mut total_distance: i64 = 0;
    points.push(pos);

    for (direction, distance) in Part2Iter::new(input) {
        let direction = match direction {
            'U' => Pos { row: -1, col: 0 },
            'D' => Pos { row: 1, col: 0 },
            'L' => Pos { row: 0, col: -1 },
            'R' => Pos { row: 0, col: 1 },
            unknown => unreachable!("Unexpected direction: {}", unknown),
        };

        pos = pos.add(&Pos {
            row: direction.row * distance,
            col: direction.col * distance,
        });
        total_distance += distance;
        points.push(pos);
    }

    shoelace(&points) + (total_distance / 2) + 1
}

#[cfg(test)]
mod tests {
    use crate::day18::{day18_part_1, day18_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day18_part_1(
                "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
            ),
            62
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day18_part_2(
                "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
            ),
            952408144115
        );
    }
}
