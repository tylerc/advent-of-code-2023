use std::collections::HashSet;

use crate::fnv1::BuildFnv1Hasher;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Pos {
    row: i64,
    col: i64,
}

impl Pos {
    fn distance(&self, other: &Pos) -> i64 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }
}

struct Galaxies {
    set: HashSet<Pos, BuildFnv1Hasher>,
    rows_populated: HashSet<i64, BuildFnv1Hasher>,
    cols_populated: HashSet<i64, BuildFnv1Hasher>,
    row_max: i64,
    col_max: i64,
}

impl Galaxies {
    fn new(capacity: usize) -> Galaxies {
        Galaxies {
            set: HashSet::with_capacity_and_hasher(capacity, BuildFnv1Hasher),
            rows_populated: HashSet::with_capacity_and_hasher(capacity / 2, BuildFnv1Hasher),
            cols_populated: HashSet::with_capacity_and_hasher(capacity / 2, BuildFnv1Hasher),
            row_max: 0,
            col_max: 0,
        }
    }

    fn insert(&mut self, pos: Pos) {
        self.set.insert(pos);
        self.rows_populated.insert(pos.row);
        self.cols_populated.insert(pos.col);
        if pos.row > self.row_max {
            self.row_max = pos.row;
        }
        if pos.col > self.col_max {
            self.col_max = pos.col;
        }
    }

    fn is_row_empty(&self, row: &i64) -> bool {
        !self.rows_populated.contains(row)
    }

    fn is_col_empty(&self, col: &i64) -> bool {
        !self.cols_populated.contains(col)
    }

    fn max(&self) -> Pos {
        Pos {
            row: self.row_max,
            col: self.col_max,
        }
    }
}

fn map_parse(input: &str) -> Galaxies {
    let mut result = Galaxies::new(450);

    for (row, line) in input.split('\n').enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                let pos = Pos {
                    row: row as i64,
                    col: col as i64,
                };
                result.insert(pos);
            }
        }
    }

    result
}

#[allow(unused_variables, clippy::let_unit_value)]
fn map_expand(map: &Galaxies, expansion: i64) -> Galaxies {
    let mut expanded_columns: Galaxies = Galaxies::new(map.set.len());
    let max = map.max();
    let mut col_dst: i64 = 0;
    for col_src in 0..=max.col {
        if map.is_col_empty(&col_src) {
            col_dst += expansion;
        } else {
            for pos_src in map.set.iter() {
                if pos_src.col == col_src {
                    expanded_columns.insert(Pos {
                        row: pos_src.row,
                        col: col_dst,
                    })
                }
            }

            col_dst += 1;
        }
    }

    // Prevent incorrect references and mutations:
    let max = Pos {
        row: max.row,
        col: col_dst,
    };
    let expanded_columns = expanded_columns;
    let col_dst = ();
    let map = ();

    let mut expanded_rows: Galaxies = Galaxies::new(expanded_columns.set.len());
    let mut row_dst: i64 = 0;
    for row_src in 0..=max.row {
        if expanded_columns.is_row_empty(&row_src) {
            row_dst += expansion;
        } else {
            for pos_src in expanded_columns.set.iter() {
                if pos_src.row == row_src {
                    expanded_rows.insert(Pos {
                        row: row_dst,
                        col: pos_src.col,
                    })
                }
            }
            row_dst += 1;
        }
    }

    expanded_rows
}

fn map_sum_distance_pairs(map: &Galaxies) -> i64 {
    let mut result = 0;
    let galaxies: Vec<_> = map.set.iter().collect();

    for (i, galaxy_a) in galaxies.iter().enumerate() {
        for galaxy_b in galaxies.iter().skip(i + 1) {
            result += galaxy_a.distance(galaxy_b);
        }
    }

    result
}

#[allow(dead_code)]
fn map_print(map: &Galaxies) {
    let max = map.max();
    for row in 0..=max.row {
        for col in 0..=max.col {
            let char = if map.set.get(&Pos { row, col }).is_some() {
                '#'
            } else {
                '.'
            };
            print!("{}", char);
        }
        println!();
    }
    println!();
}

pub fn day11_part_1(input: &str) -> i64 {
    let map = map_parse(input);
    let map = map_expand(&map, 2);
    map_sum_distance_pairs(&map)
}

pub fn day11_part_2(input: &str) -> i64 {
    let map = map_parse(input);
    let map = map_expand(&map, 1000000);
    map_sum_distance_pairs(&map)
}

#[cfg(test)]
mod tests {
    use crate::day11::{day11_part_1, map_expand, map_parse, map_sum_distance_pairs};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day11_part_1(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            ),
            374
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            map_sum_distance_pairs(&map_expand(
                &map_parse(
                    "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
                ),
                10
            )),
            1030
        );

        assert_eq!(
            map_sum_distance_pairs(&map_expand(
                &map_parse(
                    "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
                ),
                100
            )),
            8410
        );
    }
}
