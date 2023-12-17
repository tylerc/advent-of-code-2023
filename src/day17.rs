use std::collections::VecDeque;

struct City {
    heat_losses: Vec<u8>,
    rows: usize,
    cols: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: i64,
    col: i64,
}

impl Pos {
    fn turns(&self) -> (Pos, Pos) {
        match self {
            Pos { col: 0, .. } => (Pos { row: 0, col: 1 }, Pos { row: 0, col: -1 }),
            Pos { row: 0, .. } => (Pos { row: 1, col: 0 }, Pos { row: -1, col: 0 }),
            unknown => unreachable!("Invalid direction {:?}", unknown),
        }
    }

    fn is_favorable(&self) -> bool {
        self.row == 1 || self.col == 1
    }
}

#[derive(Debug)]
struct State {
    pos: Pos,
    direction: Pos,
    movement_remaining: u8,
    heat_loss: i64,
}

impl State {
    fn new(straight_line_max: u8, direction: Pos) -> State {
        State {
            pos: Pos { row: 0, col: 0 },
            direction,
            movement_remaining: straight_line_max,
            heat_loss: 0,
        }
    }

    fn turns(&self, straight_line_max: u8, heat_loss: i64) -> (State, State) {
        let directions = self.direction.turns();
        let pos1 = Pos {
            row: self.pos.row + directions.0.row,
            col: self.pos.col + directions.0.col,
        };
        let pos2 = Pos {
            row: self.pos.row + directions.1.row,
            col: self.pos.col + directions.1.col,
        };
        (
            State {
                pos: pos1,
                direction: directions.0,
                movement_remaining: straight_line_max - 1,
                heat_loss,
            },
            State {
                pos: pos2,
                direction: directions.1,
                movement_remaining: straight_line_max - 1,
                heat_loss,
            },
        )
    }
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

    fn optimal_path(&self, turn_minimum: u8, straight_line_max: u8) -> i64 {
        let mut seen: Vec<[i64; 40]> = vec![[i64::MAX; 40]; self.heat_losses.len()];
        let mut queue: VecDeque<State> = VecDeque::from([
            State::new(straight_line_max, Pos { row: 1, col: 0 }),
            State::new(straight_line_max, Pos { row: 0, col: 1 }),
        ]);
        queue[0].heat_loss -= self.heat_losses[0] as i64;
        queue[1].heat_loss -= self.heat_losses[0] as i64;

        let mut min = (self.rows as i64 - 1) * 9 + (self.cols as i64 - 1) * 9;
        while let Some(state) = queue.pop_back() {
            // We walked outside the city, skip this iteration:
            if state.pos.row < 0
                || state.pos.col < 0
                || state.pos.row as usize >= self.rows
                || state.pos.col as usize >= self.cols
            {
                continue;
            }

            let index = self.index(state.pos);
            let heat_loss = state.heat_loss + self.heat_losses[index] as i64;
            let can_stop_or_turn = straight_line_max - state.movement_remaining >= turn_minimum;

            // We reached the end:
            if can_stop_or_turn
                && state.pos.row as usize == self.rows - 1
                && state.pos.col as usize == self.cols - 1
            {
                min = min.min(heat_loss);
                continue;
            }

            // If we could never beat the min from this position, even if all remaing blocks had a
            // cost of 1, just stop:
            let best_case_min = heat_loss
                + (self.rows as i64 - state.pos.row).abs()
                + (self.cols as i64 - state.pos.col).abs();
            if best_case_min > min {
                continue;
            }

            // If we've been here before with equal or better heat_loss, skip this iteration:
            if state.movement_remaining <= 9 {
                let seen_index = state.movement_remaining as usize * 4
                    + match state.direction {
                        Pos { row: 1, col: 0 } => 0,
                        Pos { row: -1, col: 0 } => 1,
                        Pos { row: 0, col: 1 } => 2,
                        Pos { row: 0, col: -1 } => 3,
                        unknown => {
                            unreachable!(
                                "Unexpected movement remaining and direction: {:?}",
                                unknown
                            )
                        }
                    };
                if seen[index][seen_index] <= heat_loss {
                    continue;
                }

                seen[index][seen_index] = heat_loss;
            }

            // Keep going in the same direction assuming we still can:
            if state.movement_remaining > 0 {
                let state_new = State {
                    pos: Pos {
                        row: state.pos.row + state.direction.row,
                        col: state.pos.col + state.direction.col,
                    },
                    direction: state.direction,
                    movement_remaining: state.movement_remaining - 1,
                    heat_loss,
                };
                if state_new.direction.is_favorable() {
                    queue.push_back(state_new);
                } else {
                    queue.push_front(state_new);
                }
            }

            // And also turn:
            if can_stop_or_turn {
                let turns = state.turns(straight_line_max, heat_loss);
                if turns.0.direction.is_favorable() {
                    queue.push_back(turns.0);
                } else {
                    queue.push_front(turns.0);
                }
                if turns.1.direction.is_favorable() {
                    queue.push_back(turns.1);
                } else {
                    queue.push_front(turns.1);
                }
            }

            // TODO: 1. We could potentially at this point sort the queue by some heuristic
            //   or perhaps a priority queue is in order. Even a dequeue where we append down/right
            //   movement to the front might be more optimal.
        }

        min
    }
}

pub fn day17_part_1(input: &str) -> i64 {
    let city = City::new(input);
    city.optimal_path(0, 3)
}

pub fn day17_part_2(input: &str) -> i64 {
    let city = City::new(input);
    city.optimal_path(4, 10)
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
