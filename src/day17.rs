use std::collections::BinaryHeap;

struct City {
    heat_losses: Vec<u8>,
    rows: usize,
    cols: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    row: i64,
    col: i64,
}

impl Pos {
    const UP: Pos = Pos { row: -1, col: 0 };
    const DOWN: Pos = Pos { row: 1, col: 0 };
    const LEFT: Pos = Pos { row: 0, col: -1 };
    const RIGHT: Pos = Pos { row: 0, col: 1 };

    fn turns(&self) -> [Pos; 2] {
        match self {
            Pos { col: 0, .. } => [Pos::RIGHT, Pos::LEFT],
            Pos { row: 0, .. } => [Pos::DOWN, Pos::UP],
            unknown => unreachable!("Invalid direction {:?}", unknown),
        }
    }

    fn direction_index(&self) -> usize {
        match self {
            Pos { row: 1, col: 0 } => 0,
            Pos { row: -1, col: 0 } => 1,
            Pos { row: 0, col: 1 } => 2,
            Pos { row: 0, col: -1 } => 3,
            unknown => unreachable!("Invalid direction {:?}", unknown),
        }
    }

    fn add(&self, other: &Pos) -> Pos {
        Pos {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct State {
    pos: Pos,
    direction: Pos,
}

impl City {
    fn new(input: &str) -> City {
        let mut arr: Vec<u8> = Vec::new();
        let mut rows = 0;
        let mut cols = 0;

        for (row, line) in input.split('\n').enumerate() {
            for (col, char) in line.chars().enumerate() {
                let num: u8 = char.to_digit(10).expect("Input should be valid numbers.") as u8;

                arr.push(num);

                if col > cols {
                    cols = col;
                }
            }

            if row > rows {
                rows = row;
            }
        }

        // We're storing length rather than max index, so add 1:
        rows += 1;
        cols += 1;

        City {
            heat_losses: arr,
            rows,
            cols,
        }
    }

    fn index(&self, pos: Pos) -> usize {
        pos.row as usize * self.cols + pos.col as usize
    }

    fn dijkstra_path(&self, turn_minimum: u8, straight_line_max: u8) -> i64 {
        let mut cumulative_heat_losses = vec![i64::MAX; self.heat_losses.len() * 4];
        let mut processed = vec![false; self.heat_losses.len() * 4];
        let mut queue: BinaryHeap<(i64, State)> = BinaryHeap::new();

        queue.push((
            0,
            State {
                pos: Pos { row: 0, col: 0 },
                direction: Pos::RIGHT,
            },
        ));
        queue.push((
            0,
            State {
                pos: Pos { row: 0, col: 0 },
                direction: Pos::DOWN,
            },
        ));
        cumulative_heat_losses[0] = 0;
        cumulative_heat_losses[1] = 0;
        cumulative_heat_losses[2] = 0;
        cumulative_heat_losses[3] = 0;
        let goal = Pos {
            row: self.rows as i64 - 1,
            col: self.cols as i64 - 1,
        };

        // For each queue item, we immediately turn, and then add new queue items for each step
        // we could go in the direction we're facing. This means we don't have to track distance
        // traveled along a straight line, as we effectively stop at every allowed point along
        // that line.
        while let Some((_, state)) = queue.pop() {
            let meta_index = self.index(state.pos) * 4 + state.direction.direction_index();
            if processed[meta_index] {
                continue;
            }

            processed[meta_index] = true;

            let heat_loss = cumulative_heat_losses[meta_index];
            if state.pos == goal {
                return heat_loss;
            }

            let turns = state.direction.turns();

            for direction in turns.into_iter() {
                let mut new_heat_loss = heat_loss;
                let mut new_pos = state.pos;

                for i in 1..=straight_line_max {
                    new_pos = new_pos.add(&direction);
                    // Can't go out-of-bounds:
                    if new_pos.row < 0
                        || new_pos.col < 0
                        || new_pos.row >= self.rows as i64
                        || new_pos.col >= self.cols as i64
                    {
                        continue;
                    }
                    new_heat_loss += self.heat_losses[self.index(new_pos)] as i64;

                    if i >= turn_minimum {
                        let cumulative_meta_index =
                            self.index(new_pos) * 4 + direction.direction_index();
                        let existing_heat_loss = cumulative_heat_losses[cumulative_meta_index];
                        if existing_heat_loss > new_heat_loss {
                            cumulative_heat_losses[cumulative_meta_index] = new_heat_loss;
                        }

                        if !processed[cumulative_meta_index] {
                            queue.push((
                                // The queue returns largest-first, so use a negative number to
                                // reverse the usual ordering:
                                -new_heat_loss,
                                State {
                                    pos: new_pos,
                                    direction,
                                },
                            ));
                        }
                    }
                }
            }
        }

        i64::MAX
    }
}

pub fn day17_part_1(input: &str) -> i64 {
    let city = City::new(input);
    city.dijkstra_path(0, 3)
}

pub fn day17_part_2(input: &str) -> i64 {
    let city = City::new(input);
    city.dijkstra_path(4, 10)
}

#[cfg(test)]
mod tests {
    use crate::day17::{day17_part_1, day17_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day17_part_1(
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            ),
            102
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day17_part_2(
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            ),
            94
        );

        assert_eq!(
            day17_part_2(
                "111111111111
999999999991
999999999991
999999999991
999999999991"
            ),
            71
        );
    }
}
