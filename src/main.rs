use day01::day01_part_1;
use day01::day01_part_2;
use day02::day02_part_1;
use day02::day02_part_2;
use day03::day03_part_1;
use day03::day03_part_2;
use day04::day04_part_1;
use day04::day04_part_2;
use day05::day05_part_1;
use day05::day05_part_2;
use day06::day06_part_1;
use day06::day06_part_2;
use day07::day07_part_1;
use day07::day07_part_2;
use day08::day08_part_1;
use day08::day08_part_2;
use day09::day09_part_1;
use day09::day09_part_2;
use day10::day10_part_1;
use day10::day10_part_2;

use crate::day11::day11_part_1;
use crate::day11::day11_part_2;
use crate::day12::day12_part_1;
use crate::day12::day12_part_2;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod fnv1;

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
    let start = std::time::Instant::now();
    let result = func(&input);
    let elapsed = start.elapsed();
    let elapsed_str = if elapsed.as_nanos() < 1000 {
        format!("{}ns", elapsed.as_nanos())
    } else if elapsed.as_micros() < 1000 {
        format!("{}µs", elapsed.as_micros())
    } else if elapsed.as_millis() < 60000 {
        format!("{}ms", elapsed.as_millis())
    } else {
        format!("{:.2}s", elapsed.as_secs_f64())
    };

    println!(
        "| Day {:>2} | Part {} | {:>16} | {:>8} |",
        day, part, result, elapsed_str
    );
    match expected {
        None => {}
        Some(expected) => {
            assert_eq!(result, expected);
        }
    }
}

fn main() {
    println!("+--------+--------+------------------+----------+");
    execute(1, 1, day01_part_1, Some(55488));
    execute(1, 2, day01_part_2, Some(55614));
    execute(2, 1, day02_part_1, Some(2149));
    execute(2, 2, day02_part_2, Some(71274));
    execute(3, 1, day03_part_1, Some(509115));
    execute(3, 2, day03_part_2, Some(75220503));
    execute(4, 1, day04_part_1, Some(17782));
    execute(4, 2, day04_part_2, Some(8477787));
    execute(5, 1, day05_part_1, Some(551761867));
    execute(5, 2, day05_part_2, Some(57451709));
    execute(6, 1, day06_part_1, Some(2269432));
    execute(6, 2, day06_part_2, Some(35865985));
    execute(7, 1, day07_part_1, Some(245794640));
    execute(7, 2, day07_part_2, Some(247899149));
    execute(8, 1, day08_part_1, Some(20777));
    execute(8, 2, day08_part_2, Some(13289612809129));
    execute(9, 1, day09_part_1, Some(1696140818));
    execute(9, 2, day09_part_2, Some(1152));
    execute(10, 1, day10_part_1, Some(6806));
    execute(10, 2, day10_part_2, Some(449));
    execute(11, 1, day11_part_1, Some(9214785));
    execute(11, 2, day11_part_2, Some(613686987427));
    execute(12, 1, day12_part_1, Some(6935));
    execute(12, 2, day12_part_2, Some(3920437278260));
    println!("+--------+--------+------------------+----------+");
}
