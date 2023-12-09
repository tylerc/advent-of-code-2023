fn sequence_is_empty(seq: &[i64]) -> bool {
    seq.iter().all(|i| *i == 0)
}

fn sequence_to_diffs(seq: &Vec<i64>) -> Vec<i64> {
    (1..seq.len())
        .map(|i| {
            let before = seq[i - 1];
            let after = seq[i];
            after - before
        })
        .collect()
}

fn line_to_vec(line: &str) -> Vec<i64> {
    line.split(' ')
        .map(|item| item.parse().expect("Sequence numbers should be valid."))
        .collect()
}

fn line_to_all_seqs(line: &str) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = Vec::new();
    let mut current = line_to_vec(line);
    result.push(current.clone());

    while !sequence_is_empty(&current) {
        current = sequence_to_diffs(&current);
        result.push(current.clone());
    }

    result
}

fn sequences_extend(seqs: &mut [Vec<i64>]) {
    let mut diff = 0;
    for seq in seqs.iter_mut().rev() {
        seq.push(seq[seq.len() - 1] + diff);
        diff = seq[seq.len() - 1];
    }
}

fn sequences_prepend(seqs: &mut [Vec<i64>]) {
    let mut diff = 0;
    for seq in seqs.iter_mut().rev() {
        diff = seq[0] - diff;
        Vec::splice(seq, 0..1, [diff]).last();
    }
}

pub fn day09_part_1(input: &str) -> i64 {
    let mut result = 0;
    for line in input.split('\n') {
        let mut seqs = line_to_all_seqs(line);
        sequences_extend(&mut seqs);
        result += seqs[0][seqs[0].len() - 1];
    }
    result
}

pub fn day09_part_2(input: &str) -> i64 {
    let mut result = 0;
    for line in input.split('\n') {
        let mut seqs = line_to_all_seqs(line);
        sequences_prepend(&mut seqs);
        result += seqs[0][0];
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::day09::{day09_part_1, day09_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day09_part_1(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            114
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day09_part_2(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            2
        );
    }
}
