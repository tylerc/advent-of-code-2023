use day01::day01_part_1;
use day01::day01_part_2;
use day02::day02_part_1;
use day02::day02_part_2;
use day03::day03_part_1;
use day03::day03_part_2;
use day04::day04_part_1;
use day04::day04_part_2;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

fn read(path: &str) -> String {
    std::fs::read_to_string(path)
        .expect("File read to succeed.")
        .trim()
        .to_owned()
}

fn execute<T: std::fmt::Display + std::fmt::Debug + PartialEq>(
    day: i8,
    part: i8,
    func: fn(&str) -> T,
    expected: Option<T>,
) {
    let input = read(&format!("./src/day{:0>2}.txt", day));
    let result = func(&input);
    println!("Day {} - Part {}: {}", day, part, result);
    match expected {
        None => {}
        Some(expected) => {
            assert_eq!(result, expected);
        }
    }
}

fn main() {
    execute(1, 1, day01_part_1, Some(55488));
    execute(1, 2, day01_part_2, Some(55614));
    execute(2, 1, day02_part_1, Some(2149));
    execute(2, 2, day02_part_2, Some(71274));
    execute(3, 1, day03_part_1, Some(509115));
    execute(3, 2, day03_part_2, Some(75220503));
    execute(4, 1, day04_part_1, Some(17782));
    execute(4, 2, day04_part_2, Some(8477787));
}
