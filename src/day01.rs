pub fn day01_part_1(input: &str) -> i32 {
    let mut result = 0;
    for line in input.split('\n') {
        let chars: Vec<_> = line.chars().collect();
        let first = chars
            .iter()
            .find(|&c| c.is_ascii_digit())
            .expect("Expected to find the first digit.");
        let last = chars
            .iter()
            .rev()
            .find(|&c| c.is_ascii_digit())
            .expect("Expected to find the last digit.");
        let mut combined_str = String::new();
        combined_str.push(*first);
        combined_str.push(*last);
        let combined_num: i32 = combined_str.parse().expect("Expected number to be valid.");
        result += combined_num;
    }

    result
}

fn line_to_numbers(line: &str, results: &mut Vec<u8>) {
    if line.is_empty() {
        return;
    }

    let number_strs: [&str; 18] = [
        "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven",
        "8", "eight", "9", "nine",
    ];
    let numbers: [u8; 18] = [1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9];
    for (index, str) in number_strs.iter().enumerate() {
        if line.starts_with(str) {
            results.push(numbers[index]);
            break;
        }
    }

    line_to_numbers(line.split_at(1).1, results)
}

fn line_first_and_last_numbers(line: &str) -> (u8, u8) {
    let mut numbers: Vec<u8> = Vec::new();
    line_to_numbers(line, &mut numbers);
    (numbers[0], numbers[numbers.len() - 1])
}

pub fn day01_part_2(input: &str) -> i32 {
    let mut result = 0;

    for line in input.split('\n') {
        let (first, last) = line_first_and_last_numbers(line);
        let mut combined_str = String::new();
        combined_str
            .push(std::char::from_digit(first as u32, 10).expect("Valid digit as last char."));
        combined_str
            .push(std::char::from_digit(last as u32, 10).expect("Valid digit as last char."));
        let combined_num: i32 = combined_str.parse().expect("Expected number to be valid.");
        result += combined_num;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day01::{day01_part_1, day01_part_2};

    #[test]
    fn part1_example() {
        assert_eq!(
            day01_part_1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(day01_part_2("eighthree"), 83);
        assert_eq!(day01_part_2("sevenine"), 79);
        assert_eq!(
            day01_part_2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        );
    }
}
