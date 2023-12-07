use std::{collections::HashSet, hash::Hash};

#[derive(Debug)]
struct Mapping {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Range {
    range_start: i64,
    range_length: i64,
}

impl Mapping {
    fn new(input: &str) -> Mapping {
        let parsed: Vec<i64> = input
            .split(' ')
            .map(|item| {
                item.parse()
                    .unwrap_or_else(|_| panic!("Mapping numbers must be valid, got: {:?}", item))
            })
            .collect();
        Mapping {
            destination_range_start: parsed[0],
            source_range_start: parsed[1],
            range_length: parsed[2],
        }
    }

    fn src_to_dst(&self, source: i64) -> Option<i64> {
        if source >= self.source_range_start
            && source <= (self.source_range_start + self.range_length)
        {
            let offset = source - self.source_range_start;
            Some(self.destination_range_start + offset)
        } else {
            None
        }
    }

    fn clamp(&self, source: Range) -> (Option<Range>, Option<Range>, Option<Range>) {
        if source.range_start + source.range_length < self.source_range_start {
            return (Some(source), None, None);
        }

        if source.range_start > self.source_range_start + self.range_length {
            return (None, None, Some(source));
        }

        let mut body = source.clone();
        let mut head: Option<Range> = None;
        if body.range_start < self.source_range_start {
            let diff = self.source_range_start - body.range_start;
            body.range_start += diff;
            body.range_length -= diff;
            if diff > 1 {
                head = Some(Range {
                    range_start: source.range_start,
                    range_length: diff - 1,
                });
            }
        }

        let source_end = source.range_start + source.range_length;
        let mapping_end = self.source_range_start + self.range_length;
        let mut tail: Option<Range> = None;
        if source_end > mapping_end {
            let diff = source_end - mapping_end;
            body.range_length -= diff;
            if diff > 1 {
                tail = Some(Range {
                    range_start: mapping_end + 1,
                    range_length: diff - 1,
                });
            }
        }

        let offset = body.range_start - self.source_range_start;
        body.range_start = self.destination_range_start + offset;

        (head, Some(body), tail)
    }
}

#[derive(Debug)]
struct MappingGroup {
    mappings: Vec<Mapping>,
}

impl MappingGroup {
    fn new(input: &str) -> MappingGroup {
        MappingGroup {
            mappings: input.split('\n').skip(1).map(Mapping::new).collect(),
        }
    }

    fn src_to_dst(&self, source: i64) -> i64 {
        for mapping in self.mappings.iter() {
            if let Some(dst) = mapping.src_to_dst(source) {
                return dst;
            }
        }

        source
    }

    fn clamp(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut result: HashSet<Range> = HashSet::new();
        let mut to_check: HashSet<Range> = HashSet::new();

        for range in ranges {
            for mapping in self.mappings.iter() {
                let (head, body, tail) = mapping.clamp(range.clone());
                if let Some(head) = head {
                    to_check.insert(head);
                }
                if let Some(body) = body {
                    result.insert(body);
                }
                if let Some(tail) = tail {
                    to_check.insert(tail);
                }
            }
        }

        let mut seen = to_check.clone();
        while !to_check.is_empty() {
            let range = to_check
                .iter()
                .next()
                .expect("Set should have at least 1 element.")
                .clone();
            let mut found_match = false;
            let mut pieces: Vec<Range> = Vec::new();

            for mapping in self.mappings.iter() {
                let (head, body, tail) = mapping.clamp(range.clone());
                if body.is_some() {
                    found_match = true;
                    // No need to push body, it should've been covered already by our earlier loop.

                    if let Some(head) = head {
                        pieces.push(head);
                    }
                    if let Some(tail) = tail {
                        pieces.push(tail);
                    }
                }
            }

            if found_match {
                for range in pieces {
                    if !seen.contains(&range) {
                        to_check.insert(range.clone());
                        seen.insert(range);
                    }
                }
            } else {
                result.insert(range.clone());
            }
            to_check.remove(&range);
        }

        result.into_iter().collect()
    }
}

pub fn day05_part_1(input: &str) -> i64 {
    let mut lowest_location = i64::MAX;
    let mut chunks = input.split("\n\n");
    let seeds: Vec<i64> = chunks
        .next()
        .expect("Seed chunk must exist.")
        .replace("seeds: ", "")
        .split(' ')
        .map(|item| item.parse().expect("Seed number must be valid."))
        .collect();

    let mappings: Vec<_> = chunks.map(MappingGroup::new).collect();

    for seed in seeds.iter() {
        let location = mappings
            .iter()
            .fold(*seed, |last, mapping| mapping.src_to_dst(last));

        if location < lowest_location {
            lowest_location = location;
        }
    }

    lowest_location
}

pub fn day05_part_2(input: &str) -> i64 {
    let mut chunks = input.split("\n\n");
    let mut seeds: Vec<Range> = Vec::new();
    for pair in chunks
        .next()
        .expect("Seed chunk must exist.")
        .replace("seeds: ", "")
        .split(' ')
        .map(|item| item.parse::<i64>().expect("Seed number must be valid."))
        .collect::<Vec<_>>()
        .chunks(2)
    {
        seeds.push(Range {
            range_start: pair[0],
            range_length: pair[1],
        });
    }

    let mappings: Vec<_> = chunks.map(MappingGroup::new).collect();

    mappings
        .iter()
        .fold(seeds, |last, mapping| mapping.clamp(last))
        .iter()
        .min_by(|x, y| x.range_start.cmp(&y.range_start))
        .expect("Expected to find a lowest value.")
        .range_start
}

#[cfg(test)]
mod tests {
    use crate::day05::{day05_part_1, day05_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day05_part_1(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            35
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day05_part_2(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            46
        );
    }
}
