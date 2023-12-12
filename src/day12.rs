use std::collections::HashMap;

enum RecursionState {
    Continue,
    NeedDot,
    NeedHash(i64),
}

fn recurse_arrangements_memoized(
    springs: &[char],
    records: &[i64],
    state: RecursionState,
    cache: &mut HashMap<Vec<i64>, i64>,
) -> i64 {
    let mut key: Vec<i64> = springs.iter().map(|c| *c as i64).collect();
    for record in records.iter() {
        key.push(*record);
    }
    use RecursionState::*;
    key.push(match state {
        Continue => -1,
        NeedDot => -2,
        NeedHash(amount) => amount,
    });

    if let Some(result) = cache.get(&key) {
        *result
    } else {
        let result = recurse_arrangements(springs, records, state, cache);
        cache.insert(key, result);
        result
    }
}

fn recurse_arrangements(
    springs: &[char],
    records: &[i64],
    state: RecursionState,
    cache: &mut HashMap<Vec<i64>, i64>,
) -> i64 {
    use RecursionState::*;

    match state {
        Continue => {
            if springs.is_empty() && records.is_empty() {
                return 1;
            } else if springs.is_empty() {
                return 0;
            }

            match springs[0] {
                '?' => {
                    let arrangements_when_placing = if records.is_empty() {
                        0
                    } else {
                        recurse_arrangements_memoized(
                            &springs[1..],
                            &records[1..],
                            NeedHash(records[0] - 1),
                            cache,
                        )
                    };
                    let arrangements_when_not_placing =
                        recurse_arrangements_memoized(&springs[1..], records, Continue, cache);

                    arrangements_when_placing + arrangements_when_not_placing
                }
                '.' => recurse_arrangements_memoized(&springs[1..], records, Continue, cache),
                '#' => {
                    if records.is_empty() {
                        0
                    } else {
                        recurse_arrangements_memoized(
                            &springs[1..],
                            &records[1..],
                            NeedHash(records[0] - 1),
                            cache,
                        )
                    }
                }
                unknown => unreachable!("Encountered unexpected char: {}", unknown),
            }
        }
        NeedDot => {
            if springs.is_empty() {
                if records.is_empty() {
                    1
                } else {
                    0
                }
            } else if springs[0] == '#' {
                0
            } else {
                recurse_arrangements_memoized(&springs[1..], records, Continue, cache)
            }
        }
        NeedHash(0) => recurse_arrangements_memoized(springs, records, NeedDot, cache),
        NeedHash(placing) => {
            if springs.is_empty() || springs[0] == '.' {
                0
            } else {
                recurse_arrangements_memoized(&springs[1..], records, NeedHash(placing - 1), cache)
            }
        }
    }
}

pub fn day12_part_1(input: &str) -> i64 {
    let mut result: i64 = 0;
    let mut cache: HashMap<Vec<i64>, i64> = HashMap::new();

    for line in input.split('\n') {
        let split: Vec<_> = line.split(' ').collect();
        let springs: Vec<char> = split[0].chars().collect();
        let records: Vec<i64> = split[1]
            .split(',')
            .map(|item| item.parse().expect("Records should be parseable."))
            .collect();
        result += recurse_arrangements(&springs, &records, RecursionState::Continue, &mut cache)
    }

    result
}

pub fn day12_part_2(input: &str) -> i64 {
    let mut result: i64 = 0;
    let mut cache: HashMap<Vec<i64>, i64> = HashMap::new();

    for line in input.split('\n') {
        let split: Vec<_> = line.split(' ').collect();
        let mut springs: String = String::new();
        let mut records: String = String::new();
        for i in 0..5 {
            springs += split[0];
            if i != 4 {
                springs += "?";
            }
            if !records.is_empty() {
                records += ",";
            }
            records += split[1];
        }

        let springs: Vec<char> = springs.chars().collect();
        let records: Vec<i64> = records
            .split(',')
            .map(|item| item.parse().expect("Records should be parseable."))
            .collect();

        result += recurse_arrangements(&springs, &records, RecursionState::Continue, &mut cache);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day12::{day12_part_1, day12_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(day12_part_1("???.### 1,1,3"), 1);
        assert_eq!(day12_part_1(".??..??...?##. 1,1,3"), 4);
        assert_eq!(day12_part_1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(day12_part_1("????.#...#... 4,1,1"), 1);
        assert_eq!(day12_part_1("????.######..#####. 1,6,5"), 4);
        assert_eq!(day12_part_1("?###???????? 3,2,1"), 10);
        assert_eq!(
            day12_part_1(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            ),
            21
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(day12_part_2("???.### 1,1,3"), 1);
        assert_eq!(day12_part_2(".??..??...?##. 1,1,3"), 16384);
        assert_eq!(day12_part_2("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(day12_part_2("????.#...#... 4,1,1"), 16);
        assert_eq!(day12_part_2("????.######..#####. 1,6,5"), 2500);
        assert_eq!(day12_part_2("?###???????? 3,2,1"), 506250);
        assert_eq!(
            day12_part_2(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            ),
            525152
        );
    }
}
