use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    row: i64,
    col: i64,
}

#[derive(Debug, PartialEq, Eq)]
enum Rock {
    Square,
    Circle,
}
use Rock::*;

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    map: HashMap<Pos, Rock>,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut map: HashMap<Pos, Rock> = HashMap::new();
        for (row, line) in input.split('\n').enumerate() {
            for (col, char) in line.chars().enumerate() {
                if char == '.' {
                    continue;
                }

                map.insert(
                    Pos {
                        row: row as i64,
                        col: col as i64,
                    },
                    match char {
                        '#' => Square,
                        'O' => Circle,
                        unknown => unreachable!("Received unexpected character: {}", unknown),
                    },
                );
            }
        }

        Grid { map }
    }

    fn tilt_rock(&mut self, start: Pos, direction: Pos, max: Pos) {
        let rock = self.map.remove(&start).expect("Expected a rock to move.");
        let mut current = start;

        while (direction.row >= 0 || current.row > 0)
            && (direction.col >= 0 || current.col > 0)
            && (direction.row <= 0 || current.row < max.row)
            && (direction.col <= 0 || current.col < max.col)
            && self
                .map
                .get(&Pos {
                    row: current.row + direction.row,
                    col: current.col + direction.col,
                })
                .is_none()
        {
            current.row += direction.row;
            current.col += direction.col;
        }

        self.map.insert(current, rock);
    }

    fn tilt_all(&mut self, direction: Pos, max: Pos) {
        let mut temp_row_iter_1;
        let mut temp_row_iter_2;
        let row_iter: &mut dyn Iterator<Item = _> = if direction.row > 0 {
            temp_row_iter_1 = (0..max.row).rev();
            &mut temp_row_iter_1
        } else {
            temp_row_iter_2 = 0..=max.row;
            &mut temp_row_iter_2
        };

        for row in row_iter {
            let mut temp_col_iter_1;
            let mut temp_col_iter_2;
            let col_iter: &mut dyn Iterator<Item = _> = if direction.col > 0 {
                temp_col_iter_1 = (0..max.col).rev();
                &mut temp_col_iter_1
            } else {
                temp_col_iter_2 = 0..=max.col;
                &mut temp_col_iter_2
            };

            for col in col_iter {
                let pos = Pos { row, col };
                if let Some(Circle) = self.map.get(&pos) {
                    self.tilt_rock(pos, direction, max);
                }
            }
        }
    }

    fn tilt_four(&mut self, max: Pos) {
        self.tilt_all(Pos { row: -1, col: 0 }, max);
        self.tilt_all(Pos { row: 0, col: -1 }, max);
        self.tilt_all(Pos { row: 1, col: 0 }, max);
        self.tilt_all(Pos { row: 0, col: 1 }, max);
    }

    fn tilt_one_billion(&mut self, max: Pos) {
        let mut seen: HashSet<Vec<Pos>> = HashSet::new();
        let max_iters = 1000000000;
        let mut i: usize = 0;
        let mut cycle_start: Option<(usize, Vec<Pos>)> = None;
        let mut waiting_for_end = false;
        while i < max_iters {
            self.tilt_four(max);

            let mut positions: Vec<Pos> = self
                .map
                .iter()
                .flat_map(|(pos, rock)| if *rock == Circle { Some(*pos) } else { None })
                .collect();
            positions.sort();

            if seen.contains(&positions) {
                if !waiting_for_end {
                    if cycle_start.is_none() {
                        cycle_start = Some((i, positions));
                    } else if let Some((cycle_start_i, cycle_start_vec)) = &cycle_start {
                        if positions == *cycle_start_vec {
                            let cycle_size = i - cycle_start_i;
                            let remaining = max_iters - i;
                            let to_add = (remaining / cycle_size) * cycle_size;
                            i += to_add;
                            waiting_for_end = true;
                        }
                    }
                }
            } else {
                seen.insert(positions);
            }

            i += 1;
        }
    }

    fn max(&self) -> Pos {
        self.map
            .keys()
            .fold(Pos { row: 0, col: 0 }, |mut accum, pos| {
                if pos.row > accum.row {
                    accum.row = pos.row;
                }
                if pos.col > accum.col {
                    accum.col = pos.col;
                }
                accum
            })
    }

    fn score(&self) -> i64 {
        let max_row = self
            .map
            .keys()
            .map(|pos| pos.row)
            .max()
            .expect("There should be at least 1 key.");

        self.map.iter().fold(0, |accum, (pos, rock)| {
            if *rock == Square {
                accum
            } else {
                accum + ((max_row + 1) - pos.row)
            }
        })
    }

    #[allow(dead_code)]
    fn print(&self) {
        let max = self
            .map
            .keys()
            .fold(Pos { row: 0, col: 0 }, |mut accum, pos| {
                if pos.row > accum.row {
                    accum.row = pos.row;
                }
                if pos.col > accum.col {
                    accum.col = pos.col;
                }
                accum
            });

        for row in 0..=max.row {
            for col in 0..=max.col {
                print!(
                    "{}",
                    match self.map.get(&Pos { row, col }) {
                        Some(Square) => '#',
                        Some(Circle) => 'O',
                        None => '.',
                    }
                );
            }
            println!();
        }
        println!();
    }
}

pub fn day14_part_1(input: &str) -> i64 {
    let mut grid = Grid::new(input);
    let max = grid.max();
    grid.tilt_all(Pos { row: -1, col: 0 }, max);
    grid.score()
}

pub fn day14_part_2(input: &str) -> i64 {
    let mut grid = Grid::new(input);
    let max = grid.max();
    grid.tilt_one_billion(max);
    grid.score()
}

#[cfg(test)]
mod tests {
    use crate::day14::day14_part_1;

    use super::Grid;

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day14_part_1(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            ),
            136
        );
    }

    #[test]
    pub fn part2_example() {
        let mut grid = Grid::new(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        let max = grid.max();
        grid.tilt_four(max);
        assert_eq!(
            grid,
            Grid::new(
                ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
            )
        );

        grid.tilt_four(max);
        assert_eq!(
            grid,
            Grid::new(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
            )
        );

        grid.tilt_four(max);
        assert_eq!(
            grid,
            Grid::new(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
            )
        );

        grid = Grid::new(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        grid.tilt_one_billion(max);

        assert_eq!(grid.score(), 64);
    }
}
