#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mirror {
    Empty,
    Right,
    Left,
    Horizontal,
    Vertical,
}

use Mirror::*;

struct Contraption {
    grid: Vec<Mirror>,
    energized: Vec<u8>,
    rows: usize,
    cols: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: i64,
    col: i64,
}

impl Contraption {
    fn new(input: &str) -> Contraption {
        let mut grid: Vec<Mirror> = Vec::new();
        let mut energized: Vec<u8> = Vec::new();
        let mut rows = 0;
        let mut cols = 0;

        for (row, line) in input.split('\n').enumerate() {
            for (col, char) in line.chars().enumerate() {
                grid.push(match char {
                    '.' => Empty,
                    '/' => Right,
                    '\\' => Left,
                    '-' => Horizontal,
                    '|' => Vertical,
                    unknown => unreachable!("Encountered unexpected character: {}", unknown),
                });
                energized.push(0);

                if col > cols {
                    cols = col;
                }
            }

            if row > rows {
                rows = row;
            }
        }

        // 0-based indexing means we need to add one more to get the total count:
        rows += 1;
        cols += 1;

        Contraption {
            grid,
            energized,
            rows,
            cols,
        }
    }

    fn get(&self, pos: Pos) -> Mirror {
        self.grid[pos.row as usize * self.rows + pos.col as usize]
    }

    fn energize(&mut self, pos: Pos, direction: Pos) -> bool {
        let index = pos.row as usize * self.rows + pos.col as usize;
        let bits = match direction {
            Pos { row: 1, col: 0 } => 0b1,
            Pos { row: -1, col: 0 } => 0b10,
            Pos { row: 0, col: 1 } => 0b100,
            Pos { row: 0, col: -1 } => 0b1000,
            unexpected => unreachable!("Unexpected direction energized {:?}", unexpected),
        };
        let existing = self.energized[index];
        if existing & bits > 0 {
            return true;
        }

        self.energized[index] = existing | bits;
        false
    }

    fn simulate_beams(&mut self, start_location: Pos, start_direction: Pos) -> usize {
        let mut queue = vec![(start_location, start_direction)];
        self.energized = vec![0; self.energized.len()];

        while let Some((pos, direction)) = queue.pop() {
            if pos.row >= 0
                && pos.col >= 0
                && pos.row < self.rows as i64
                && pos.col < self.cols as i64
            {
                let seen = self.energize(pos, direction);
                if seen {
                    continue;
                }
            }

            let pos = Pos {
                row: pos.row + direction.row,
                col: pos.col + direction.col,
            };
            if pos.row < 0
                || pos.col < 0
                || pos.row >= self.rows as i64
                || pos.col >= self.cols as i64
            {
                // Beam has left the grid, stop simulating:
                continue;
            }

            let mirror = self.get(pos);
            if mirror == Empty
                || (mirror == Horizontal && direction.col != 0)
                || (mirror == Vertical && direction.row != 0)
            {
                queue.push((pos, direction));
                continue;
            }

            if mirror == Horizontal {
                queue.push((pos, Pos { row: 0, col: -1 }));
                queue.push((pos, Pos { row: 0, col: 1 }));
            } else if mirror == Vertical {
                queue.push((pos, Pos { row: -1, col: 0 }));
                queue.push((pos, Pos { row: 1, col: 0 }));
            } else if mirror == Right {
                queue.push((
                    pos,
                    match direction {
                        Pos { row: 1, col: 0 } => Pos { row: 0, col: -1 },
                        Pos { row: -1, col: 0 } => Pos { row: 0, col: 1 },
                        Pos { row: 0, col: 1 } => Pos { row: -1, col: 0 },
                        Pos { row: 0, col: -1 } => Pos { row: 1, col: 0 },
                        unknown => unreachable!(
                            "Hit a right-angled mirror with an unacceptable direction {:?}",
                            unknown
                        ),
                    },
                ));
            } else if mirror == Left {
                queue.push((
                    pos,
                    match direction {
                        Pos { row: 1, col: 0 } => Pos { row: 0, col: 1 },
                        Pos { row: -1, col: 0 } => Pos { row: 0, col: -1 },
                        Pos { row: 0, col: 1 } => Pos { row: 1, col: 0 },
                        Pos { row: 0, col: -1 } => Pos { row: -1, col: 0 },
                        unknown => unreachable!(
                            "Hit a left-angled mirror with an unacceptable direction {:?}",
                            unknown
                        ),
                    },
                ));
            } else {
                unreachable!("Mirror has an impossible value {:?}", mirror);
            }
        }

        self.energized_total()
    }

    fn optimal_beam(&mut self) -> usize {
        let mut max = 0;

        for col in 0..self.cols {
            max = max.max(self.simulate_beams(
                Pos {
                    row: -1,
                    col: col as i64,
                },
                Pos { row: 1, col: 0 },
            ));

            max = max.max(self.simulate_beams(
                Pos {
                    row: self.rows as i64,
                    col: col as i64,
                },
                Pos { row: -1, col: 0 },
            ));
        }

        for row in 0..self.rows {
            max = max.max(self.simulate_beams(
                Pos {
                    row: row as i64,
                    col: -1,
                },
                Pos { row: 0, col: 1 },
            ));

            max = max.max(self.simulate_beams(
                Pos {
                    row: row as i64,
                    col: self.cols as i64,
                },
                Pos { row: 0, col: -1 },
            ));
        }

        max
    }

    fn energized_total(&self) -> usize {
        self.energized
            .iter()
            .fold(0, |accum, item| if *item > 0 { accum + 1 } else { accum })
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        for (index, mirror) in self.grid.iter().enumerate() {
            if index > 0 && index % self.cols == 0 {
                println!();
            }

            print!(
                "{}",
                match mirror {
                    Empty => '.',
                    Left => '\\',
                    Right => '/',
                    Horizontal => '-',
                    Vertical => '|',
                }
            );
        }
        println!("\n");
    }

    #[allow(dead_code)]
    fn print_energized(&self) {
        for (index, is_energized) in self.energized.iter().enumerate() {
            if index > 0 && index % self.cols == 0 {
                println!();
            }

            print!("{}", if *is_energized > 0 { '#' } else { '.' });
        }
        println!("\n");
    }
}

pub fn day16_part_1(input: &str) -> usize {
    let mut contraption = Contraption::new(input);
    contraption.simulate_beams(Pos { row: 0, col: -1 }, Pos { row: 0, col: 1 })
}

pub fn day16_part_2(input: &str) -> usize {
    let mut contraption = Contraption::new(input);
    contraption.simulate_beams(Pos { row: 0, col: -1 }, Pos { row: 0, col: 1 });
    contraption.optimal_beam()
}

#[cfg(test)]
mod tests {
    use crate::day16::{day16_part_1, day16_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day16_part_1(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            ),
            46
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day16_part_2(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            ),
            51
        );
    }
}
