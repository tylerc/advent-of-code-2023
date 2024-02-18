use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}
use Tile::*;

use crate::fnv1::BuildFnv1Hasher;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(PartialEq, Eq)]
struct Journey {
    pos: Pos,
    cost: usize,
    visited: HashSet<Pos, BuildFnv1Hasher>,
}

impl Ord for Journey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost
            .cmp(&other.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Journey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Journey {
    fn is_done(&self, world: &World) -> bool {
        self.pos == world.destination
    }
}

struct World {
    tiles: Vec<Tile>,
    highest_costs: Vec<usize>,
    edges: BinaryHeap<Journey>,
    destination: Pos,
    rows: usize,
    cols: usize,
}

impl World {
    fn new(input: &str) -> World {
        let mut tiles: Vec<Tile> = Vec::new();
        let mut rows = 0;
        let mut cols = 0;

        for line in input.split('\n') {
            cols = 0;

            for char in line.chars() {
                tiles.push(match char {
                    '.' => Path,
                    '#' => Forest,
                    '^' => Slope(Up),
                    'v' => Slope(Down),
                    '<' => Slope(Left),
                    '>' => Slope(Right),
                    unknown => unreachable!("Encountered unexpected tile {}", unknown),
                });
                cols += 1;
            }

            rows += 1;
        }

        let start = Pos { row: 0, col: 1 };
        let mut visited = HashSet::with_hasher(BuildFnv1Hasher);
        visited.insert(start);
        let highest_costs: Vec<usize> = vec![0; tiles.len()];
        let mut edges: BinaryHeap<Journey> = BinaryHeap::new();
        edges.push(Journey {
            pos: Pos { row: 0, col: 1 },
            cost: 0,
            visited,
        });

        World {
            tiles,
            highest_costs,
            edges,
            destination: Pos {
                row: rows - 1,
                col: cols - 2,
            },
            rows,
            cols,
        }
    }

    fn pos_to_index(&self, pos: &Pos) -> usize {
        pos.col + pos.row * self.cols
    }

    fn pos_add(&self, pos: &Pos, dir: Direction) -> Option<Pos> {
        let mut result: Pos = *pos;
        match dir {
            Up => {
                if pos.row == 0 {
                    return None;
                }
                result.row -= 1;
            }
            Down => {
                if pos.row == self.rows - 1 {
                    return None;
                }
                result.row += 1;
            }
            Left => {
                if pos.col == 0 {
                    return None;
                }
                result.col -= 1;
            }
            Right => {
                if pos.col == self.cols - 1 {
                    return None;
                }
                result.col += 1;
            }
        }

        Some(result)
    }

    fn destinations(&self, pos: &Pos, follow_slopes: bool) -> Vec<Pos> {
        let index = self.pos_to_index(pos);
        let tile = self.tiles[index];
        [Up, Down, Left, Right]
            .into_iter()
            .filter(|dir| match (tile, follow_slopes) {
                (Path, _) => true,
                (Slope(slope_dir), true) => *dir == slope_dir,
                (Slope(_), false) => true,
                (Forest, _) => false,
            })
            .filter_map(|dir| {
                let maybe_pos = self.pos_add(pos, dir);
                if let Some(pos) = maybe_pos {
                    if let Forest = self.tiles[self.pos_to_index(&pos)] {
                        return None;
                    }
                }
                maybe_pos
            })
            .collect()
    }

    fn to_graph(&self, follow_slopes: bool) -> HashMap<Pos, Vec<(usize, Pos)>, BuildFnv1Hasher> {
        let mut result: HashMap<Pos, Vec<(usize, Pos)>, BuildFnv1Hasher> =
            HashMap::with_hasher(BuildFnv1Hasher);
        let mut to_check = vec![Pos { row: 0, col: 1 }];

        while let Some(pos) = to_check.pop() {
            if result.contains_key(&pos) {
                continue;
            }

            let destinations: Vec<(usize, Pos)> = self
                .destinations(&pos, follow_slopes)
                .into_iter()
                .filter_map(|dest| {
                    let mut dest_using = dest;
                    let mut seen: HashSet<Pos, BuildFnv1Hasher> =
                        HashSet::with_hasher(BuildFnv1Hasher);
                    seen.insert(pos);
                    let mut cost = 1;
                    loop {
                        seen.insert(dest_using);
                        if dest_using == self.destination {
                            break;
                        }

                        let connections: Vec<Pos> = self
                            .destinations(&dest_using, follow_slopes)
                            .into_iter()
                            .filter(|dest2| !seen.contains(dest2))
                            .collect();

                        match connections.len() {
                            // This is a dead end, just prune this whole path:
                            0 => return None,
                            // 1 connection, so we'll skip forward:
                            1 => {
                                dest_using = connections[0];
                                cost += 1;
                            }
                            _ => {
                                // There's never any point walking past the exit (you'll just block it),
                                // so if we're connected to the exit at all, pretend it's a direct connection.
                                for dest2 in connections.iter() {
                                    if *dest2 == self.destination {
                                        dest_using = self.destination;
                                        cost += 1;
                                        break;
                                    }
                                }

                                // More than 1 connection, we'll want to branch when walking the graph:
                                break;
                            }
                        }
                    }

                    Some((cost, dest_using))
                })
                .collect();

            for (_, pos2) in destinations.iter() {
                to_check.push(*pos2);
            }

            result.insert(pos, destinations);
        }

        result
    }

    fn find_costs_with_graph(&mut self, follow_slopes: bool) -> usize {
        let heuristic_offset = if follow_slopes { 0 } else { 1900 };
        let graph = self.to_graph(follow_slopes);
        let mut destination_costliest_journey = Journey {
            pos: self.destination,
            cost: 0,
            visited: HashSet::with_hasher(BuildFnv1Hasher),
        };

        while let Some(journey) = self.edges.pop() {
            if journey.is_done(self) {
                if destination_costliest_journey.cost < journey.cost {
                    destination_costliest_journey = journey;
                }
                continue;
            }

            let destinations = graph
                .get(&journey.pos)
                .expect("Expected graph to contain every relevant position.");
            for (cost_incr, dest) in destinations.iter() {
                let next_cost = journey.cost + cost_incr;

                if journey.visited.contains(dest) {
                    continue;
                }

                let index = self.pos_to_index(dest);
                if self.highest_costs[index] < next_cost + heuristic_offset {
                    self.highest_costs[index] = next_cost;
                    let mut visited = journey.visited.clone();
                    visited.insert(*dest);
                    self.edges.push(Journey {
                        pos: *dest,
                        cost: next_cost,
                        visited,
                    })
                }
            }
        }

        destination_costliest_journey.cost
    }
}

pub fn day23_part_1(input: &str) -> i64 {
    World::new(input).find_costs_with_graph(true) as i64
}

pub fn day23_part_2(input: &str) -> i64 {
    World::new(input).find_costs_with_graph(false) as i64
}

#[cfg(test)]
mod tests {
    use crate::day23::day23_part_1;
    use crate::day23::day23_part_2;

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day23_part_1(
                "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
            ),
            94
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day23_part_2(
                "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
            ),
            154
        );
    }
}
