use std::{collections::HashSet, str::FromStr};

#[derive(Clone, Copy, PartialEq, Debug)]
struct Pos<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Hailstone<T> {
    pos: Pos<T>,
    velocity: Pos<T>,
}

impl<T: FromStr + Copy> Hailstone<T> {
    fn new(line: &str) -> Hailstone<T> {
        let halves: Vec<T> = line
            .replace(" @ ", ", ")
            .split(", ")
            .map(|str| {
                str.trim()
                    .parse()
                    .unwrap_or_else(|_| panic!("Expected number to be valid, got: \"{}\"", str))
            })
            .collect();
        Hailstone {
            pos: Pos {
                x: halves[0],
                y: halves[1],
                z: halves[2],
            },
            velocity: Pos {
                x: halves[3],
                y: halves[4],
                z: halves[5],
            },
        }
    }
}

impl Hailstone<f64> {
    fn x_to_time(&self, x: f64) -> f64 {
        (x - self.pos.x) / self.velocity.x
    }

    fn standard_form(&self) -> (f64, f64, f64) {
        let a = (self.pos.y + self.velocity.y) - self.pos.y;
        let b = self.pos.x - (self.pos.x + self.velocity.x);
        let c = a * self.pos.x + b * self.pos.y;
        (a, b, c)
    }

    // From https://www.topcoder.com/thrive/articles/Geometry%20Concepts%20part%202:%20%20Line%20Intersection%20and%20its%20Applications
    // See also https://rosettacode.org/wiki/Find_the_intersection_of_two_lines
    fn point_of_intersection(&self, other: &Hailstone<f64>) -> Option<(f64, f64)> {
        let (a1, b1, c1) = self.standard_form();
        let (a2, b2, c2) = other.standard_form();
        let det = a1 * b2 - a2 * b1;
        if det == 0.0 {
            return None;
        }
        let x = (b2 * c1 - b1 * c2) / det;
        let y = (a1 * c2 - a2 * c1) / det;
        Some((x, y))
    }
}

pub fn day24_part_1_general(input: &str, start: f64, end: f64) -> i64 {
    let hailstones: Vec<Hailstone<f64>> = input.split('\n').map(Hailstone::new).collect();
    let mut result: i64 = 0;

    for (index, h1) in hailstones.iter().enumerate() {
        for h2 in hailstones.iter().skip(index + 1) {
            if let Some((x, y)) = h1.point_of_intersection(h2) {
                if x >= start && x <= end && y >= start && y <= end {
                    let t1 = h1.x_to_time(x);
                    let t2 = h2.x_to_time(x);
                    if t1 < 0.0 || t2 < 0.0 {
                        continue;
                    }

                    result += 1;
                }
            }
        }
    }

    result
}

pub fn day24_part_1(input: &str) -> i64 {
    day24_part_1_general(input, 200000000000000.0, 400000000000000.0)
}

fn will_collide(x1: i64, v1: i64, x2: i64, v2: i64) -> bool {
    // If they're stationary relative to each other, they will never collide:
    let v_abs = (v1 - v2).abs();
    if v_abs == 0 {
        // Unless they were sitting at the same coordinate the whole time:
        if x1 == x2 {
            return true;
        }

        return false;
    }

    // We can't start on top of a hailstone, and so if the absolute velocities,
    // are different then starting at the same coordinate means we never collide:
    let x_abs = (x1 - x2).abs();
    if x_abs == 0 {
        return false;
    }

    // If the difference in position isn't a multiple of the absolute velocity,
    // one will jump past the other:
    if x_abs % v_abs != 0 {
        return false;
    }

    // If one has already overtaken the other (i.e. collision would've happened in the past if ever),
    // they will never collide:
    if (v1 > v2 && x1 > x2) || (v2 > v1 && x2 > x1) {
        return false;
    }

    true
}

#[derive(Debug)]
struct Constraint {
    min: Option<i64>,
    max: Option<i64>,
}

impl Constraint {
    fn combine(&self, other: &Constraint) -> Option<Constraint> {
        let result = Constraint {
            min: match (self.min, other.min) {
                (Some(n1), Some(n2)) => Some(n1.max(n2)),
                (Some(n), None) => Some(n),
                (None, Some(n)) => Some(n),
                (None, None) => None,
            },
            max: match (self.max, other.max) {
                (Some(n1), Some(n2)) => Some(n1.min(n2)),
                (Some(n), None) => Some(n),
                (None, Some(n)) => Some(n),
                (None, None) => None,
            },
        };

        if let (Some(min), Some(max)) = (result.min, result.max) {
            if min > max {
                return None;
            }
        }

        Some(result)
    }
}

fn hailstone_and_velocity_to_constraint(
    hailstone_pos: i64,
    hailstone_vel: i64,
    rock_vel: i64,
    time: Option<i64>,
) -> Constraint {
    let vel_abs = (hailstone_vel - rock_vel).abs();
    if vel_abs == 0 {
        // If they're stationary relative to each other, the rock _has_ to start with the same position as this hailstone:
        Constraint {
            min: Some(hailstone_pos),
            max: Some(hailstone_pos),
        }
    } else if let Some(time) = time {
        // If we know the collision time, we can compute the exact starting position of the rock necessary to hit at this velocity:
        let pos_at_time = hailstone_pos + hailstone_vel * time;
        let rock_orig_pos = pos_at_time - rock_vel * time;
        Constraint {
            min: Some(rock_orig_pos),
            max: Some(rock_orig_pos),
        }
    } else if hailstone_vel > rock_vel {
        // If the hailstone is going faster than the rock, the rock needs to start a bit ahead of the hailstone:
        Constraint {
            min: Some(hailstone_pos + vel_abs),
            max: None,
        }
    } else {
        // If the hailstone is going slower than the rock, the rock needs to start a bit behind the hailstone:
        Constraint {
            min: None,
            max: Some(hailstone_pos - vel_abs),
        }
    }
}

// We consider one axis at a time, knowing that solving one axis makes solving
// the other axes trivial. We make two assumptions:
//
// 1. The rock velocity is likely to be similar in magnitude to the hailstone velocity.
// 2. When considering the viable starting positions at a certain velocity, it will
//    be possible to narrow down an axis to one concrete starting position that works
//    for all hailstones.
//
// #1 proved true for all axes, #2 proved true for the Y axis.
fn find_on_axis(
    hailstones_initial: &[Hailstone<i64>],
    get_pos: fn(&Hailstone<i64>) -> i64,
    get_vel: fn(&Hailstone<i64>) -> i64,
    get_time: &dyn Fn(&Hailstone<i64>) -> Option<i64>,
) -> Option<(i64, i64)> {
    let known_vels: HashSet<i64> = hailstones_initial.iter().map(get_vel).collect();
    let largest_magnitude = known_vels
        .iter()
        .map(|vel| vel.abs())
        .max()
        .expect("There should be a max.");
    let min = -largest_magnitude * 2;
    let max = largest_magnitude * 2;

    'velocity_loop: for rock_vel in min..=max {
        let mut constraint = Constraint {
            min: None,
            max: None,
        };
        // For this velocity, find the ranges of potentially valid starting positions:
        for hailstone in hailstones_initial.iter() {
            let time = get_time(hailstone);
            let new_constraint = hailstone_and_velocity_to_constraint(
                get_pos(hailstone),
                get_vel(hailstone),
                rock_vel,
                time,
            );
            if let Some(combined) = constraint.combine(&new_constraint) {
                constraint = combined;
            } else {
                // If the range is invalid, we need to check another velocity:
                continue 'velocity_loop;
            }
        }

        // The key hope: that one of the ranges will collapse down to a single value:
        if let Some(rock_pos) = constraint.min {
            if constraint.min == constraint.max
                && hailstones_initial
                    .iter()
                    .all(|h| will_collide(get_pos(h), get_vel(h), rock_pos, rock_vel))
            {
                return Some((rock_pos, rock_vel));
            }
        }
    }

    None
}

pub fn day24_part_2(input: &str) -> i64 {
    let hailstones_initial: Vec<Hailstone<i64>> = input.split('\n').map(Hailstone::new).collect();
    // For both the test and real inputs, the Y axis can be found easily:
    let (rock_pos_y, rock_vel_y) =
        find_on_axis(&hailstones_initial, |h| h.pos.y, |h| h.velocity.y, &|_| {
            None
        })
        .expect("Expected to find initial y.");

    // And once we have the starting Y position and velocity, we can
    // easily compute collision time for all hailstones:
    let hailstone_to_collision_time = |h: &Hailstone<i64>| {
        // It's fine if we can't find every single one, all other collisions will
        // force us into a single starting position even without these:
        if rock_vel_y == h.velocity.y {
            return None;
        }

        Some((h.pos.y - rock_pos_y) / (rock_vel_y - h.velocity.y))
    };

    let (rock_pos_x, _) = find_on_axis(
        &hailstones_initial,
        |h| h.pos.x,
        |h| h.velocity.x,
        &hailstone_to_collision_time,
    )
    .expect("Expected to find initial x.");

    let (rock_pos_z, _) = find_on_axis(
        &hailstones_initial,
        |h| h.pos.z,
        |h| h.velocity.z,
        &hailstone_to_collision_time,
    )
    .expect("Expected to find initial z.");

    rock_pos_x + rock_pos_y + rock_pos_z
}

#[cfg(test)]
mod tests {
    use crate::day24::day24_part_1_general;
    use crate::day24::day24_part_2;

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day24_part_1_general(
                "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3",
                7.0,
                27.0
            ),
            2
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day24_part_2(
                "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
            ),
            24 + 13 + 10
        );
    }
}
