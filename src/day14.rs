use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    row: i64,
    col: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Rock {
    Nothing,
    Square,
    Circle,
}
use Rock::*;

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    arr: Vec<Rock>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut arr: Vec<Rock> = Vec::new();
        let mut rows = 0;
        let mut cols = 0;

        for (row, line) in input.split('\n').enumerate() {
            for (col, char) in line.chars().enumerate() {
                if char == '.' {
                    arr.push(Nothing);
                } else if char == '#' {
                    arr.push(Square);
                } else if char == 'O' {
                    arr.push(Circle);
                } else {
                    unreachable!("Encountered unexpected character {}", char);
                }

                if col > cols {
                    cols = col;
                }
            }

            if row > rows {
                rows = row;
            }
        }

        // Index starts at 0 and we want a total count, so add 1:
        rows += 1;
        cols += 1;

        Grid { arr, rows, cols }
    }

    fn pos_to_index(&self, pos: Pos) -> usize {
        (pos.row as usize) * self.cols + (pos.col as usize)
    }

    fn tilt_rock(&mut self, start: Pos, direction: Pos, max: Pos) {
        let start_index = self.pos_to_index(start);
        let rock = self.arr[start_index];
        self.arr[start_index] = Nothing;
        let mut current = start;

        while (direction.row >= 0 || current.row > 0)
            && (direction.col >= 0 || current.col > 0)
            && (direction.row <= 0 || current.row < max.row)
            && (direction.col <= 0 || current.col < max.col)
            && self.arr[self.pos_to_index(Pos {
                row: current.row + direction.row,
                col: current.col + direction.col,
            })] == Nothing
        {
            current.row += direction.row;
            current.col += direction.col;
        }

        let current_index = self.pos_to_index(current);
        self.arr[current_index] = rock;
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
                if self.arr[self.pos_to_index(pos)] == Circle {
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
        let mut seen: HashSet<Vec<Rock>> = HashSet::new();
        let max_iters = 1000000000;
        let mut i: usize = 0;
        let mut cycle_start: Option<(usize, Vec<Rock>)> = None;
        let mut waiting_for_end = false;
        while i < max_iters {
            self.tilt_four(max);

            if seen.contains(&self.arr) {
                if !waiting_for_end {
                    if cycle_start.is_none() {
                        cycle_start = Some((i, self.arr.clone()));
                    } else if let Some((cycle_start_i, cycle_start_vec)) = &cycle_start {
                        if self.arr == *cycle_start_vec {
                            let cycle_size = i - cycle_start_i;
                            let remaining = max_iters - i;
                            let to_add = (remaining / cycle_size) * cycle_size;
                            i += to_add;
                            waiting_for_end = true;
                        }
                    }
                }
            } else {
                seen.insert(self.arr.clone());
            }

            i += 1;
        }
    }

    fn max(&self) -> Pos {
        Pos {
            row: self.rows as i64 - 1,
            col: self.cols as i64 - 1,
        }
    }

    fn score(&self) -> i64 {
        self.arr.iter().enumerate().fold(0, |accum, (index, rock)| {
            if *rock == Circle {
                let row = index / self.rows;
                accum + ((self.rows as i64) - row as i64)
            } else {
                accum
            }
        })
    }

    #[allow(dead_code)]
    fn print(&self) {
        let max = self.max();

        for row in 0..=max.row {
            for col in 0..=max.col {
                print!(
                    "{}",
                    match self.arr[self.pos_to_index(Pos { row, col })] {
                        Square => '#',
                        Circle => 'O',
                        Nothing => '.',
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
