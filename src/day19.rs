use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy)]
#[repr(usize)]
enum Category {
    ExtremelyCool = 0usize,
    Musical = 1usize,
    Aerodynamic = 2usize,
    Shiny = 3usize,
}
use Category::*;

enum Op {
    LessThan,
    GreaterThan,
}
use Op::*;

struct Rule<'a> {
    category: Category,
    op: Op,
    rhs: i64,
    destination: &'a str,
}

struct RuleList<'a> {
    rules: Vec<Rule<'a>>,
    final_destination: &'a str,
}

struct Part {
    categories: [i64; 4],
}

fn day19_parse<'a>(input: &'a str) -> (HashMap<&str, RuleList<'a>>, Vec<Part>) {
    let mut halves = input.split("\n\n");
    let rules_text = halves.next().expect("Expected to find list of rules.");
    let parts_text = halves.next().expect("Expected to find list of parts.");
    let rule_regexp =
        Regex::new(r"^([xmas]+)([<>])(\d+):([a-zA-Z]+)$").expect("Regexp should be valid.");

    let mut rule_lists: HashMap<&str, RuleList<'a>> = HashMap::new();

    for line in rules_text.split('\n') {
        let mut name_and_remainder = line.split('{');
        let name = name_and_remainder.next().expect("Expected to find name.");
        let remainder = name_and_remainder
            .next()
            .expect("Expected to find rule list.");
        let mut list = RuleList {
            rules: Vec::new(),
            final_destination: "",
        };

        let rule_strs: Vec<_> = remainder.split(',').collect();
        for rule_str in rule_strs.iter().take(rule_strs.len() - 1) {
            let (_, [category_str, op_str, rhs_str, destination]) = rule_regexp
                .captures(rule_str)
                .expect("Regexp should match.")
                .extract();

            let category = match category_str {
                "x" => ExtremelyCool,
                "m" => Musical,
                "a" => Aerodynamic,
                "s" => Shiny,
                unknown => unreachable!("Encounted unexpected category: {}", unknown),
            };

            let op = match op_str {
                "<" => LessThan,
                ">" => GreaterThan,
                unknown => unreachable!("Encountered unexpected op: {}", unknown),
            };

            let rhs: i64 = rhs_str
                .parse()
                .expect("Expected rule to have a i64 right-hand side.");

            list.rules.push(Rule {
                category,
                op,
                rhs,
                destination,
            });
        }
        list.final_destination = rule_strs[rule_strs.len() - 1].trim_end_matches('}');

        rule_lists.insert(name, list);
    }

    let mut parts: Vec<Part> = Vec::new();

    let parts_regexp = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$")
        .expect("Expect parts regexp to be valid.");
    for line in parts_text.split('\n') {
        let [x, m, a, s] = parts_regexp
            .captures(line)
            .expect("Expect parts regexp to match.")
            .extract()
            .1
            .map(|num_str| {
                num_str
                    .parse::<i64>()
                    .expect("Parts numbers should be valid.")
            });

        parts.push(Part {
            categories: [x, m, a, s],
        });
    }

    (rule_lists, parts)
}

pub fn day19_part_1(input: &str) -> i64 {
    let (rule_lists, parts) = day19_parse(input);

    parts
        .into_iter()
        .filter(|p| {
            let mut rule_list_current = rule_lists
                .get("in")
                .expect("Initial rule should always be found.");

            'outer: loop {
                for rule in rule_list_current.rules.iter() {
                    if match rule.op {
                        GreaterThan => p.categories[rule.category as usize] > rule.rhs,
                        LessThan => p.categories[rule.category as usize] < rule.rhs,
                    } {
                        if rule.destination == "A" {
                            return true;
                        } else if rule.destination == "R" {
                            return false;
                        } else {
                            rule_list_current = rule_lists
                                .get(rule.destination)
                                .expect("Destination rule should always be found.");
                            continue 'outer;
                        }
                    }
                }

                if rule_list_current.final_destination == "A" {
                    return true;
                } else if rule_list_current.final_destination == "R" {
                    return false;
                } else {
                    rule_list_current = rule_lists
                        .get(rule_list_current.final_destination)
                        .expect("Final destination should always be found.");
                }
            }
        })
        .map(|p| p.categories.into_iter().sum::<i64>())
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new() -> Range {
        Range {
            start: 1,
            end: 4000,
        }
    }
}

pub fn day19_part_2(input: &str) -> i64 {
    let rules_lists = day19_parse(input).0;
    let start = rules_lists
        .get("in")
        .expect("Starting rule should be found.");
    let range_initial: [Range; 4] = [Range::new(), Range::new(), Range::new(), Range::new()];
    let mut approved_ranges: Vec<[Range; 4]> = Vec::new();
    let mut queue: Vec<([Range; 4], &RuleList)> = vec![(range_initial, start)];

    while let Some((range, rule_list)) = queue.pop() {
        let mut range_not_matched = range;

        for rule in rule_list.rules.iter() {
            let matched = match rule.op {
                LessThan => {
                    let mut range_matched = range_not_matched;

                    if range_matched[rule.category as usize].end >= rule.rhs {
                        range_matched[rule.category as usize].end = rule.rhs - 1;
                    }

                    if range_not_matched[rule.category as usize].start < rule.rhs {
                        range_not_matched[rule.category as usize].start = rule.rhs;
                    }

                    range_matched
                }
                GreaterThan => {
                    let mut range_matched = range_not_matched;

                    if range_matched[rule.category as usize].start <= rule.rhs {
                        range_matched[rule.category as usize].start = rule.rhs + 1;
                    }

                    if range_not_matched[rule.category as usize].end > rule.rhs {
                        range_not_matched[rule.category as usize].end = rule.rhs;
                    }

                    range_matched
                }
            };

            if rule.destination == "A" {
                approved_ranges.push(matched);
            } else if rule.destination != "R" {
                queue.push((
                    matched,
                    rules_lists
                        .get(rule.destination)
                        .expect("Expected to find destination."),
                ));
            }
        }

        if rule_list.final_destination == "A" {
            approved_ranges.push(range_not_matched);
        } else if rule_list.final_destination != "R" {
            queue.push((
                range_not_matched,
                rules_lists
                    .get(rule_list.final_destination)
                    .expect("Expected to find final destination."),
            ));
        }
    }

    let mut possibilities: i64 = 0;
    for ranges in approved_ranges {
        let mut range_combinations = 0;
        for (i, range) in ranges.into_iter().enumerate() {
            if i == 0 {
                range_combinations = 1 + range.end - range.start;
            } else {
                range_combinations *= 1 + range.end - range.start;
            }
        }
        possibilities += range_combinations;
    }

    possibilities
}

#[cfg(test)]
mod tests {
    use crate::day19::{day19_part_1, day19_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day19_part_1(
                "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            ),
            19114
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day19_part_2(
                "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            ),
            167409079868000
        );
    }
}
