use day01::day01_part_1;
use day01::day01_part_2;

pub mod day01;

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
}
