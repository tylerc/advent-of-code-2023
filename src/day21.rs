#[derive(Clone, Copy)]
enum Tile {
    GardenPlot,
    Rock,
}
use std::collections::HashSet;

use Tile::*;

use crate::fnv1::BuildFnv1Hasher;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    row: i64,
    col: i64,
}

impl Pos {
    fn add(&self, other: Pos) -> Pos {
        Pos {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

struct Garden {
    grid: Vec<Tile>,
    occupied: HashSet<Pos>,
    rows: i64,
    cols: i64,
    wrap: bool,
}

impl Garden {
    fn new(input: &str, wrap: bool) -> Garden {
        let mut grid: Vec<Tile> = Vec::new();
        let mut rows: i64 = 0;
        let mut cols: i64 = 0;
        let mut start = Pos { row: -1, col: -1 };

        for (row, line) in input.split('\n').enumerate() {
            let row = row as i64;
            if row > rows {
                rows = row;
            }

            for (col, char) in line.chars().enumerate() {
                let col = col as i64;
                if col > cols {
                    cols = col;
                }

                let pos = Pos { row, col };
                let tile = match char {
                    'S' => {
                        start = pos;
                        GardenPlot
                    }

                    '.' => GardenPlot,
                    '#' => Rock,
                    unknown => unreachable!("Encountered invalid char: {}", unknown),
                };
                grid.push(tile);
            }
        }

        rows += 1;
        cols += 1;

        let mut occupied = HashSet::new();
        occupied.insert(start);

        Garden {
            grid,
            occupied,
            rows,
            cols,
            wrap,
        }
    }

    fn pos_to_index(&self, pos: Pos) -> usize {
        let mut pos = pos;
        if self.wrap {
            if pos.row < 0 {
                pos.row = pos.row % self.rows + self.rows;
            }
            if pos.col < 0 {
                pos.col = pos.col % self.cols + self.cols;
            }
            if pos.row >= self.rows {
                pos.row %= self.rows;
            }
            if pos.col >= self.cols {
                pos.col %= self.cols;
            }
        } else if pos.row < 0 || pos.col < 0 || pos.row >= self.cols || pos.col >= self.cols {
            return usize::MAX;
        }

        (pos.row * self.cols + pos.col) as usize
    }

    fn get_tile(&self, pos: Pos) -> Tile {
        self.grid[self.pos_to_index(pos)]
    }

    fn valid_moves(&self, pos: Pos) -> [Option<Pos>; 4] {
        [
            Pos { row: -1, col: 0 },
            Pos { row: 1, col: 0 },
            Pos { row: 0, col: -1 },
            Pos { row: 0, col: 1 },
        ]
        .map(|offset| {
            let destination = pos.add(offset);
            let index = self.pos_to_index(destination);

            if index >= self.grid.len() {
                return None;
            }

            if let GardenPlot = self.get_tile(destination) {
                Some(destination)
            } else {
                None
            }
        })
    }

    fn iterate(&mut self) {
        let mut occupied_new: HashSet<Pos> = HashSet::new();

        for pos in self.occupied.iter() {
            for destination in self.valid_moves(*pos).into_iter().flatten() {
                occupied_new.insert(destination);
            }
        }

        self.occupied = occupied_new;
    }

    fn iterate_many(&mut self, mut count: usize) -> usize {
        if count % 2 == 1 {
            self.iterate();
            count -= 1;
        }
        let mut visited = HashSet::with_capacity_and_hasher(count, BuildFnv1Hasher);
        let mut to_process: Vec<Pos> = self.occupied.iter().cloned().collect();
        let mut to_process_next: Vec<Pos> = Vec::new();

        let mut count = count as i64;
        while count >= 0 && (!to_process.is_empty() || !to_process_next.is_empty()) {
            while let Some(pos) = to_process.pop() {
                if visited.contains(&pos) {
                    continue;
                }
                visited.insert(pos);

                for destination_a in self.valid_moves(pos).into_iter().flatten() {
                    for destination_b in self.valid_moves(destination_a).into_iter().flatten() {
                        if destination_b != pos && !visited.contains(&destination_b) {
                            to_process_next.push(destination_b);
                        }
                    }
                }
            }

            count -= 2;

            std::mem::swap(&mut to_process, &mut to_process_next);
        }

        visited.len()
    }

    fn score_part_1(&self) -> i64 {
        self.occupied.len() as i64
    }
}

pub fn day21_part_1(input: &str) -> i64 {
    let mut garden = Garden::new(input, false);
    let goal_steps = if garden.cols == 11 { 6 } else { 64 };

    for _ in 0..goal_steps {
        garden.iterate();
    }

    garden.score_part_1()
}

// Honestly, this part 2 was a bit beyond me. After bashing my head against it for hours
// I had to look up what others had done. Apparently lagrange interpolation on these
// 3 points yields the correct result for the input (but not the example). I don't fully
// understand it all, but at this point I'm happy to collect my stars and be done.
pub fn day21_part_2(input: &str) -> f64 {
    let mut points: Vec<(f64, f64)> = Vec::new();
    for count in [65, 131 + 65, 131 * 2 + 65] {
        let mut garden = Garden::new(input, true);
        let result = garden.iterate_many(count);
        points.push((count as f64, result as f64));
    }

    let mut result: f64 = 0.0;
    for i in 0..3 {
        let mut term: f64 = points[i].1;

        for j in 0..3 {
            if i != j {
                term = term * (26501365f64 - points[j].0) / (points[i].0 - points[j].0);
            }
        }

        result += term;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day21::day21_part_1;

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day21_part_1(
                "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
            ),
            16
        );
    }
}
