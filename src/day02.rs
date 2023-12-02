pub fn day02_part_1(input: &str) -> u32 {
    let red_contained = 12;
    let green_contained = 13;
    let blue_contained = 14;

    let mut game_ids_possible_total = 0;

    for line in input.split('\n') {
        let game_and_details: Vec<_> = line.split(": ").collect();
        let game_str = game_and_details[0];
        let details_str = game_and_details[1];

        let game_id: u32 = game_str
            .split(' ')
            .nth(1)
            .expect("A game ID should exist.")
            .parse()
            .expect("A game ID should be a valid u32.");

        let rounds = details_str.split(';');
        let mut all_rounds_valid = true;
        for round in rounds {
            let mut red_known = 0;
            let mut green_known = 0;
            let mut blue_known = 0;

            for piece in round.split(',') {
                let mut word_and_count_iter = piece.trim().split(' ');
                let count: u32 = word_and_count_iter
                    .next()
                    .expect("Expected a count string.")
                    .parse()
                    .expect("A count should be a valid u32.");
                let color: &str = word_and_count_iter.next().expect("A color should exist.");

                match color {
                    "red" => red_known += count,
                    "green" => green_known += count,
                    "blue" => blue_known += count,
                    _ => panic!(
                        "Expected known color, got {} (with count: {})",
                        color, count
                    ),
                }
            }

            if red_known > red_contained
                || green_known > green_contained
                || blue_known > blue_contained
            {
                all_rounds_valid = false;
                break;
            }
        }

        if all_rounds_valid {
            game_ids_possible_total += game_id;
        }
    }

    game_ids_possible_total
}

pub fn day02_part_2(input: &str) -> u32 {
    let mut sum_of_powers = 0;

    for line in input.split('\n') {
        let details_str = line.split(": ").nth(1).expect("Expected a details string.");
        let rounds = details_str.split(';');

        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        for round in rounds {
            let mut red_known = 0;
            let mut green_known = 0;
            let mut blue_known = 0;

            for piece in round.split(',') {
                let mut word_and_count_iter = piece.trim().split(' ');
                let count: u32 = word_and_count_iter
                    .next()
                    .expect("Expected a count string.")
                    .parse()
                    .expect("A count should be a valid u32.");
                let color: &str = word_and_count_iter.next().expect("A color should exist.");

                match color {
                    "red" => red_known += count,
                    "green" => green_known += count,
                    "blue" => blue_known += count,
                    _ => panic!(
                        "Expected known color, got {} (with count: {})",
                        color, count
                    ),
                }
            }

            red_max = std::cmp::max(red_max, red_known);
            green_max = std::cmp::max(green_max, green_known);
            blue_max = std::cmp::max(blue_max, blue_known);
        }

        sum_of_powers += red_max * green_max * blue_max;
    }

    sum_of_powers
}

#[cfg(test)]
mod tests {
    use crate::day02::{day02_part_1, day02_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day02_part_1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day02_part_2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }
}
