use regex::Regex;

pub fn day06_part_1(input: &str) -> i64 {
    let re = Regex::new(r"\s+").expect("Invalid regex.");
    let lines: Vec<_> = input.split('\n').collect();
    let times: Vec<i64> = re
        .split(lines[0])
        .skip(1)
        .map(|num_str| {
            num_str
                .parse()
                .expect("Expected time to be a valid number.")
        })
        .collect();

    let distances: Vec<i64> = re
        .split(lines[1])
        .skip(1)
        .map(|num_str| {
            num_str
                .parse()
                .expect("Expected distance to be a valid number.")
        })
        .collect();

    let mut result = 1;

    for i in 0..times.len() {
        let time_available = times[i];
        let distance_to_beat = distances[i];

        let mut ways_to_win = 0;
        for time_pressed in 1..time_available {
            let distance_moved = time_pressed * (time_available - time_pressed);
            if distance_moved > distance_to_beat {
                ways_to_win += 1;
            }
        }

        result *= ways_to_win;
    }

    result
}

pub fn day06_part_2(input: &str) -> i64 {
    let re = Regex::new(r"\s+").expect("Invalid regex.");
    let lines: Vec<_> = input.split('\n').collect();
    let time: i64 = re
        .replace_all(lines[0], "")
        .split(':')
        .nth(1)
        .expect("Expected a time value.")
        .parse()
        .expect("Expected a valid time number.");
    let distance: i64 = re
        .replace_all(lines[1], "")
        .split(':')
        .nth(1)
        .expect("Expected a distance value.")
        .parse()
        .expect("Expected a valid distance number.");

    let mut ways_to_win = 0;
    for time_pressed in 1..time {
        let distance_moved = time_pressed * (time - time_pressed);
        if distance_moved > distance {
            ways_to_win += 1;
        }
    }

    ways_to_win
}

#[cfg(test)]
mod tests {
    use crate::day06::{day06_part_1, day06_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day06_part_1(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            288
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day06_part_2(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            71503
        );
    }
}
