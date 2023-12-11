use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
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

    fn distance(&self, other: &Pos) -> i64 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }
}

type Galaxies = HashMap<Pos, bool>;

fn map_parse(input: &str) -> Galaxies {
    let mut result: Galaxies = HashMap::new();

    for (row, line) in input.split('\n').enumerate() {
        for (col, char) in line.chars().enumerate() {
            let pos = Pos {
                row: row as i64,
                col: col as i64,
            };
            match char {
                '.' => result.insert(pos, false),
                '#' => result.insert(pos, true),
                _ => unreachable!("Unexpected character: {}", char),
            };
        }
    }

    result
}

fn is_empty(map: &Galaxies, start: Pos, end: Pos, offset: Pos) -> bool {
    let mut current = start;
    while current != end.add(&offset) {
        if let Some(is_galaxy) = map.get(&current) {
            if *is_galaxy {
                return false;
            }
        }

        current = current.add(&offset);
    }
    true
}

#[allow(unused_variables, clippy::let_unit_value)]
fn map_expand(map: &Galaxies, expansion: i64) -> Galaxies {
    let mut expanded_columns: Galaxies = HashMap::new();
    let max = map
        .keys()
        .max()
        .expect("Expected to find maximum coordinates.");
    let mut col_dst: i64 = 0;
    for col_src in 0..=max.col {
        if is_empty(
            map,
            Pos {
                row: 0,
                col: col_src,
            },
            Pos {
                row: max.row,
                col: col_src,
            },
            Pos { row: 1, col: 0 },
        ) {
            col_dst += expansion;
        } else {
            for (row_dst, row_src) in (0_i64..).zip(0..=max.row) {
                if let Some(galaxy_or_space) = map.get(&Pos {
                    row: row_src,
                    col: col_src,
                }) {
                    expanded_columns.insert(
                        Pos {
                            row: row_dst,
                            col: col_dst,
                        },
                        *galaxy_or_space,
                    );
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

    let mut expanded_rows: Galaxies = HashMap::new();
    let mut row_dst: i64 = 0;
    for row_src in 0..=max.row {
        if is_empty(
            &expanded_columns,
            Pos {
                row: row_src,
                col: 0,
            },
            Pos {
                row: row_src,
                col: max.col,
            },
            Pos { row: 0, col: 1 },
        ) {
            row_dst += expansion;
        } else {
            for (col_dst, col_src) in (0_i64..).zip(0..=max.col) {
                if let Some(galaxy_or_space) = expanded_columns.get(&Pos {
                    row: row_src,
                    col: col_src,
                }) {
                    expanded_rows.insert(
                        Pos {
                            row: row_dst,
                            col: col_dst,
                        },
                        *galaxy_or_space,
                    );
                }
            }
            row_dst += 1;
        }
    }

    expanded_rows
}

fn map_sum_distance_pairs(map: &Galaxies) -> i64 {
    let mut result = 0;
    let galaxies: Vec<_> = map
        .iter()
        .filter(|(_, is_galaxy)| **is_galaxy)
        .map(|(pos, _)| pos)
        .collect();

    for (i, galaxy_a) in galaxies.iter().enumerate() {
        for galaxy_b in galaxies.iter().skip(i + 1) {
            result += galaxy_a.distance(galaxy_b);
        }
    }

    result
}

#[allow(dead_code)]
fn map_print(map: &Galaxies) {
    let max = map.keys().max().expect("Expected to find max coordinate.");
    for row in 0..=max.row {
        for col in 0..=max.col {
            let char = map
                .get(&Pos { row, col })
                .map(|is_galaxy| if *is_galaxy { '#' } else { '.' })
                .unwrap_or('*');
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
