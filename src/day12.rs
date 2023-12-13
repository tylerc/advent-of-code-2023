use std::collections::HashMap;

const KNOWN_GOOD: u8 = 46; // ASCII .
const KNOWN_BAD: u8 = 35; // ASCII #
const UNKNOWN: u8 = 63; // ASCII ?

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum RecursionState {
    Continue,
    NeedHash(RecordInt),
}

type SpringType = u8;
type RecordInt = u8;
type CacheMap<'a> = HashMap<(&'a [SpringType], &'a [RecordInt], RecursionState), u64>;

fn recurse_arrangements_memoized<'a>(
    springs: &'a [SpringType],
    records: &'a [RecordInt],
    state: RecursionState,
    cache: &mut CacheMap<'a>,
) -> u64 {
    let key = (springs, records, state);
    if let Some(result) = cache.get(&key) {
        *result
    } else {
        let result = recurse_arrangements(springs, records, state, cache);
        cache.insert(key, result);
        result
    }
}

fn recurse_arrangements<'a>(
    springs: &'a [SpringType],
    records: &'a [RecordInt],
    state: RecursionState,
    cache: &mut CacheMap<'a>,
) -> u64 {
    use RecursionState::*;

    match state {
        Continue => {
            if springs.is_empty() && records.is_empty() {
                return 1;
            } else if springs.is_empty() {
                return 0;
            }

            match springs[0] {
                UNKNOWN => {
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
                KNOWN_GOOD => {
                    recurse_arrangements_memoized(&springs[1..], records, Continue, cache)
                }
                KNOWN_BAD => {
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
                bad => unreachable!("Unexpected character: {}", bad),
            }
        }
        NeedHash(placing) => {
            // If we're out of items to place, we need to find a dot or the end of the input:
            if placing == 0 {
                if springs.is_empty() {
                    if records.is_empty() {
                        1
                    } else {
                        0
                    }
                } else if springs[0] == KNOWN_BAD {
                    0
                } else {
                    recurse_arrangements_memoized(&springs[1..], records, Continue, cache)
                }
            } else if springs.is_empty() || springs[0] == KNOWN_GOOD {
                0
            } else {
                recurse_arrangements_memoized(&springs[1..], records, NeedHash(placing - 1), cache)
            }
        }
    }
}

pub fn day12_part_1(input: &str) -> u64 {
    let mut result: u64 = 0;

    for line in input.split('\n') {
        let mut cache: CacheMap = HashMap::new();
        let split: Vec<_> = line.split(' ').collect();
        let records: Vec<RecordInt> = split[1]
            .split(',')
            .map(|item| item.parse().expect("Records should be parseable."))
            .collect();
        result += recurse_arrangements(
            split[0].as_bytes(),
            &records,
            RecursionState::Continue,
            &mut cache,
        )
    }

    result
}

pub fn day12_part_2(input: &str) -> u64 {
    use std::sync::mpsc::{channel, Receiver, Sender};
    use std::thread;

    let thread_count = 16;
    let mut result: u64 = 0;

    let (transmit_results, receive_results) = channel::<u64>();
    let mut transmit_work: Vec<Sender<String>> = Vec::new();
    let mut receive_work: Vec<Receiver<String>> = Vec::new();

    for _ in 0..thread_count {
        let (tx, rx) = channel::<String>();
        transmit_work.push(tx);
        receive_work.push(rx);
    }

    for receiver in receive_work {
        let transmit_results = transmit_results.clone();
        thread::spawn(move || {
            while let Ok(line) = receiver.recv() {
                let mut cache: CacheMap = HashMap::new();
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

                let records: Vec<RecordInt> = records
                    .split(',')
                    .map(|item| item.parse().expect("Records should be parseable."))
                    .collect();

                transmit_results
                    .send(recurse_arrangements(
                        springs.as_bytes(),
                        &records,
                        RecursionState::Continue,
                        &mut cache,
                    ))
                    .expect("Results should be sent successfully.");
            }
        });
    }
    drop(transmit_results);

    for (index, line) in input.split('\n').enumerate() {
        transmit_work[index % thread_count]
            .send(line.to_owned())
            .expect("Work should be sent successfully.");
    }

    drop(transmit_work);
    while let Ok(answer) = receive_results.recv() {
        result += answer;
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
