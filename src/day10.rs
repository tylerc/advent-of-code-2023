/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has. */

use std::collections::{HashMap, HashSet};

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

    fn subtract(&self, other: &Pos) -> Pos {
        Pos {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

fn pipe_to_connections(pipe: char) -> Vec<Pos> {
    match pipe {
        '|' => vec![Pos { row: -1, col: 0 }, Pos { row: 1, col: 0 }],
        '-' => vec![Pos { row: 0, col: 1 }, Pos { row: 0, col: -1 }],
        'L' => vec![Pos { row: -1, col: 0 }, Pos { row: 0, col: 1 }],
        'J' => vec![Pos { row: -1, col: 0 }, Pos { row: 0, col: -1 }],
        '7' => vec![Pos { row: 1, col: 0 }, Pos { row: 0, col: -1 }],
        'F' => vec![Pos { row: 1, col: 0 }, Pos { row: 0, col: 1 }],
        '.' => Vec::with_capacity(0),
        'S' => Vec::with_capacity(0),
        _ => unreachable!("Encountered unexpected connection char: {}", pipe),
    }
}

#[derive(Debug)]
struct Pipe {
    pos: Pos,
    kind: char,
    connections: Vec<Pos>,
}

fn map_parse(input: &str) -> (HashMap<Pos, Pipe>, Pos) {
    let mut result: HashMap<Pos, Pipe> = HashMap::new();
    let mut start_pos: Option<Pos> = None;

    for (row, line) in input.split('\n').enumerate() {
        for (col, char) in line.chars().enumerate() {
            let pos = Pos {
                row: row as i64,
                col: col as i64,
            };

            result.insert(
                pos,
                Pipe {
                    pos,
                    kind: char,
                    connections: pipe_to_connections(char)
                        .into_iter()
                        .map(|offset| pos.add(&offset))
                        .collect(),
                },
            );

            if char == 'S' {
                start_pos = Some(pos);
            }
        }
    }

    let start_pos = start_pos.expect("Expected to find starting position.");
    let surrounding = [
        Pos { row: -1, col: 0 },
        Pos { row: 0, col: 1 },
        Pos { row: 1, col: 0 },
        Pos { row: 0, col: -1 },
    ]
    .iter()
    .map(|item| start_pos.add(item));

    let mut starting_connections: Vec<Pos> = Vec::with_capacity(2);
    for pos in surrounding {
        if let Some(pipe) = result.get(&pos) {
            for connection in pipe.connections.iter() {
                if *connection == start_pos {
                    starting_connections.push(pos);
                }
            }
        }
    }

    result
        .get_mut(&start_pos)
        .expect("Expected to find Pipe for starting Pos.")
        .connections = starting_connections;

    (result, start_pos)
}

fn distance_count(map: &HashMap<Pos, Pipe>, start_pos: Pos) -> HashMap<Pos, i64> {
    let mut result: HashMap<Pos, i64> = HashMap::new();
    let mut to_visit: HashSet<Pos> = HashSet::new();
    result.insert(start_pos, 0);
    to_visit.insert(start_pos);

    while !to_visit.is_empty() {
        let pos = to_visit
            .iter()
            .next()
            .cloned()
            .expect("Expected to find Pos to visit.");
        to_visit.remove(&pos);
        let distance = *result
            .get(&pos)
            .expect("Expected to find distance to current Pos.");
        distance_count_visitor(map, &mut result, &mut to_visit, pos, distance);
    }

    result
}

fn distance_count_visitor(
    map: &HashMap<Pos, Pipe>,
    visited: &mut HashMap<Pos, i64>,
    to_visit: &mut HashSet<Pos>,
    current: Pos,
    current_distance_to_start: i64,
) {
    let connection_distance_to_start = current_distance_to_start + 1;
    for connection in map
        .get(&current)
        .expect("Expected to find connected pipe.")
        .connections
        .iter()
    {
        let previous_distance = visited.get(connection).unwrap_or(&i64::MAX);
        if connection_distance_to_start < *previous_distance {
            visited.insert(*connection, connection_distance_to_start);
            to_visit.insert(*connection);
        }
    }
}

fn map_extend(map: &mut HashMap<Pos, Pipe>) {
    let mut min = Pos {
        row: i64::MAX,
        col: i64::MAX,
    };
    let mut max = Pos {
        row: i64::MIN,
        col: i64::MIN,
    };

    for pos in map.keys() {
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
    min.row -= 1;
    min.col -= 1;
    max.row += 1;
    max.col += 1;

    for col in min.col..=max.col {
        // Top:
        let pos = Pos { row: min.row, col };
        map.insert(
            pos,
            Pipe {
                kind: 'O',
                pos,
                connections: Vec::with_capacity(0),
            },
        );

        // Bottom:
        let pos = Pos { row: max.row, col };
        map.insert(
            pos,
            Pipe {
                kind: 'O',
                pos,
                connections: Vec::with_capacity(0),
            },
        );
    }

    for row in min.row..=max.row {
        // Left:
        let pos = Pos { row, col: min.col };
        map.insert(
            pos,
            Pipe {
                kind: 'O',
                pos,
                connections: Vec::with_capacity(0),
            },
        );

        // Right:
        let pos = Pos { row, col: max.col };
        map.insert(
            pos,
            Pipe {
                kind: 'O',
                pos,
                connections: Vec::with_capacity(0),
            },
        );
    }
}

fn map_double(map: &HashMap<Pos, Pipe>) -> HashMap<Pos, Pipe> {
    let mut needs_adjustments: Vec<Pos> = Vec::new();
    let mut result: HashMap<Pos, Pipe> = map
        .iter()
        .map(|(pos, pipe)| {
            let pos_new = Pos {
                row: pos.row * 2,
                col: pos.col * 2,
            };
            needs_adjustments.push(pos_new);
            (
                pos_new,
                Pipe {
                    kind: pipe.kind,
                    pos: pos_new,
                    connections: pipe
                        .connections
                        .iter()
                        .map(|pos| Pos {
                            row: pos.row * 2,
                            col: pos.col * 2,
                        })
                        .collect(),
                },
            )
        })
        .collect();

    let max = *result
        .keys()
        .max()
        .expect("Expect to be able to find max key after doubling.");

    for pos in needs_adjustments {
        let pipe = result
            .get(&pos)
            .expect("Pipe just added to map should still be there.");
        let mut pipes_needed: Vec<Pos> = Vec::new();
        for connection in pipe.connections.iter() {
            let mut diff = connection.subtract(&pos);
            diff.row /= 2;
            diff.col /= 2;
            pipes_needed.push(Pos {
                row: pos.row + diff.row,
                col: pos.col + diff.col,
            });
        }
        for new_pos in pipes_needed {
            if new_pos.row >= 0
                && new_pos.col >= 0
                && new_pos.row <= max.row
                && new_pos.col <= max.col
            {
                result.insert(
                    new_pos,
                    Pipe {
                        kind: if new_pos.row == pos.row { '-' } else { '|' },
                        pos: new_pos,
                        // We'll never use these connections, so don't bother creating them:
                        connections: vec![],
                    },
                );
            }
        }
    }

    for row in 0..=max.row {
        for col in 0..=max.col {
            let pos = Pos { row, col };
            result.entry(pos).or_insert(Pipe {
                kind: '.',
                pos,
                connections: vec![],
            });
        }
    }

    result
}

fn map_halve(map: &HashMap<Pos, Pipe>) -> HashMap<Pos, Pipe> {
    map.iter()
        .filter(|(pos, _)| pos.row % 2 == 0 && pos.col % 2 == 0)
        .map(|(pos, pipe)| {
            let pos_new = Pos {
                row: pos.row / 2,
                col: pos.col / 2,
            };
            (
                pos_new,
                Pipe {
                    kind: pipe.kind,
                    pos: pos_new,
                    connections: pipe
                        .connections
                        .iter()
                        .map(|pos| Pos {
                            row: pos.row / 2,
                            col: pos.col / 2,
                        })
                        .collect(),
                },
            )
        })
        .collect()
}

fn map_flood_fill(map: &mut HashMap<Pos, Pipe>) {
    let mut to_visit: HashSet<(Pos, Option<Vec<Pos>>)> = HashSet::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    for pipe in map.values() {
        if pipe.kind == 'O' {
            to_visit.insert((pipe.pos, None));
        }
    }

    while !to_visit.is_empty() {
        let entry = to_visit
            .iter()
            .next()
            .cloned()
            .expect("Expected to find a Pos to flood-fill from.");
        to_visit.remove(&entry);
        let (pos, directions) = entry;
        visited.insert(pos);

        let surrounding_offets = if let Some(directions) = &directions {
            directions.iter()
        } else {
            [
                Pos { row: -1, col: 0 },
                Pos { row: 0, col: 1 },
                Pos { row: 1, col: 0 },
                Pos { row: 0, col: -1 },
            ]
            .iter()
        };

        let surrounding = surrounding_offets.map(|item| pos.add(item));

        for pos in surrounding {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            if let Some(pipe) = map.get_mut(&pos) {
                if pipe.kind == '.' {
                    to_visit.insert((pipe.pos, None));
                    pipe.kind = 'O';
                }
            }
        }
    }
}

fn map_remove_junk(map: &mut HashMap<Pos, Pipe>, start_pos: Pos) {
    let loop_positions: HashSet<Pos> = distance_count(map, start_pos).keys().cloned().collect();
    for (pos, pipe) in map.iter_mut() {
        if !loop_positions.contains(pos) {
            pipe.kind = '.';
            pipe.connections = vec![];
        }
    }
}

pub fn day10_part_1(input: &str) -> i64 {
    let (map, start_pos) = map_parse(input);
    let distances = distance_count(&map, start_pos);
    *distances
        .values()
        .max()
        .expect("Expected to find maximum distance.")
}

pub fn day10_part_2(input: &str) -> i64 {
    let (mut map, start_pos) = map_parse(input);
    map_remove_junk(&mut map, start_pos);
    let mut doubled = map_double(&map);
    map_extend(&mut doubled);
    map_flood_fill(&mut doubled);
    let halved = map_halve(&doubled);
    halved.values().fold(
        0,
        |accum, pipe| {
            if pipe.kind == '.' {
                accum + 1
            } else {
                accum
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::day10::{day10_part_1, day10_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day10_part_1(
                ".....
.S-7.
.|.|.
.L-J.
....."
            ),
            4
        );
        assert_eq!(
            day10_part_1(
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            4
        );
        assert_eq!(
            day10_part_1(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            ),
            8
        );
        assert_eq!(
            day10_part_1(
                "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            ),
            8
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day10_part_2(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            ),
            4
        );

        assert_eq!(
            day10_part_2(
                "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
            ),
            4
        );

        assert_eq!(
            day10_part_2(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            8
        );

        assert_eq!(
            day10_part_2(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            10
        );
    }
}
