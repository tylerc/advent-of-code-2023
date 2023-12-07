use regex::Regex;
use std::collections::HashSet;

pub fn day04_part_1(input: &str) -> i32 {
    let mut score: i32 = 0;
    let re = Regex::new(r"\s+").expect("Invalid regex.");

    for line in input.lines() {
        let lists_str = line
            .split(": ")
            .nth(1)
            .expect("Expected to split line on ': ' properly.");
        let lists: Vec<_> = lists_str.split(" | ").collect();
        let winning_numbers: HashSet<i32> = re
            .split(lists[0].trim())
            .map(|item| {
                item.parse()
                    .expect("Expected a valid winning number to parse.")
            })
            .collect();

        score += re.split(lists[1].trim()).fold(0, |accum, item| {
            let num: i32 = item
                .trim()
                .parse()
                .expect("Expected a valid possessed number to parse.");
            if winning_numbers.contains(&num) {
                if accum == 0 {
                    1
                } else {
                    accum * 2
                }
            } else {
                accum
            }
        });
    }

    score
}

pub fn day04_part_2(input: &str) -> i32 {
    let line_count = input.lines().count();
    let mut total_scratchcards = line_count as i32;
    let mut iterations_by_card_number: Vec<i32> = vec![1; line_count];
    let re = Regex::new(r"\s+").expect("Invalid regex.");

    for line in input.lines() {
        let mut prefix_and_postfix = line.split(": ");
        let card_index: usize = prefix_and_postfix
            .next()
            .expect("Must have card number.")
            .replace("Card ", "")
            .trim()
            .parse::<usize>()
            .expect("Card number should be a valid number.")
            - 1;

        let lists: Vec<_> = prefix_and_postfix
            .next()
            .expect("Must have numbers.")
            .split(" | ")
            .collect();
        let winning_numbers: HashSet<i32> = re
            .split(lists[0].trim())
            .map(|item| {
                item.parse()
                    .expect("Expected a valid winning number to parse.")
            })
            .collect();

        let mut card_gaining = card_index + 1;
        for num_str in re.split(lists[1].trim()) {
            let num: i32 = num_str
                .trim()
                .parse()
                .expect("Expected a valid possessed number to parse.");

            if winning_numbers.contains(&num) {
                total_scratchcards += iterations_by_card_number[card_index];
                iterations_by_card_number[card_gaining] += iterations_by_card_number[card_index];
                card_gaining += 1;
            }
        }
    }

    total_scratchcards
}

#[cfg(test)]
mod tests {
    use crate::day04::{day04_part_1, day04_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day04_part_1(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            13
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day04_part_2(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            30
        );
    }
}
