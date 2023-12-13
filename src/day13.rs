fn row_diffs(lines: &[&[u8]], row_a: usize, row_b: usize) -> usize {
    let mut diffs = 0;

    for col in 0..lines[0].len() {
        if lines[row_a][col] != lines[row_b][col] {
            diffs += 1;
        }
    }

    diffs
}

fn has_horizontal_symmetry(lines: &[&[u8]], diffs_expected: usize) -> Option<usize> {
    'outer: for bottom_row in (1..lines.len()).rev() {
        let top_row = bottom_row - 1;
        let max_movement = top_row.min(lines.len() - bottom_row - 1);
        let mut diffs = 0;
        for i in 0..=max_movement {
            diffs += row_diffs(lines, top_row - i, bottom_row + i);
            if diffs > diffs_expected {
                continue 'outer;
            }
        }

        if diffs == diffs_expected {
            return Some(bottom_row);
        }
    }

    None
}

pub fn transform(lines: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    for row in 0..lines[0].len() {
        let mut row_result: Vec<u8> = Vec::new();
        for line in lines {
            row_result.push(line[row]);
        }
        result.push(row_result);
    }
    result
}

pub fn day13_part_1(input: &str) -> usize {
    let mut result: usize = 0;

    for group in input.split("\n\n") {
        let lines: Vec<_> = group.split('\n').map(|str| str.as_bytes()).collect();
        if let Some(rows_above) = has_horizontal_symmetry(lines.as_slice(), 0) {
            result += rows_above * 100;
        } else {
            let transformed = transform(lines.as_slice());
            let transformed: Vec<_> = transformed.iter().map(|item| item.as_slice()).collect();
            if let Some(cols_left) = has_horizontal_symmetry(transformed.as_slice(), 0) {
                result += cols_left;
            }
        }
    }

    result
}

pub fn day13_part_2(input: &str) -> usize {
    let mut result: usize = 0;

    for group in input.split("\n\n") {
        let lines: Vec<_> = group.split('\n').map(|str| str.as_bytes()).collect();
        if let Some(rows_above) = has_horizontal_symmetry(lines.as_slice(), 1) {
            result += rows_above * 100;
        } else {
            let transformed = transform(lines.as_slice());
            let transformed: Vec<_> = transformed.iter().map(|item| item.as_slice()).collect();
            if let Some(cols_left) = has_horizontal_symmetry(transformed.as_slice(), 1) {
                result += cols_left;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day13::{day13_part_1, day13_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day13_part_1(
                "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            400
        );

        assert_eq!(
            day13_part_1(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            ),
            5
        );

        assert_eq!(
            day13_part_1(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            405
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day13_part_2(
                "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            100
        );

        assert_eq!(
            day13_part_2(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            ),
            300
        );

        assert_eq!(
            day13_part_2(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            400
        );
    }
}
