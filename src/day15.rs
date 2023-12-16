fn hash(input: &str) -> usize {
    let mut result: usize = 0;

    for char in input.bytes() {
        result += char as usize;
        result *= 17;
        result %= 256;
    }

    result
}

struct HashMap<'a> {
    slots: [Vec<(&'a str, u8)>; 256],
}

impl<'a> HashMap<'a> {
    fn new() -> HashMap<'a> {
        HashMap {
            slots: core::array::from_fn(|_| Vec::new()),
        }
    }

    fn insert(&mut self, label: &'a str, value: u8) {
        let index = hash(label);
        for (other_label, other_value) in self.slots[index].iter_mut() {
            if *other_label == label {
                *other_value = value;
                return;
            }
        }

        self.slots[index].push((label, value));
    }

    fn remove(&mut self, label: &str) {
        let slot_index = hash(label);
        let mut vec_index: Option<usize> = None;
        for (index, (other_label, _)) in self.slots[slot_index].iter().enumerate() {
            if *other_label == label {
                vec_index = Some(index);
                break;
            }
        }

        if let Some(vec_index) = vec_index {
            self.slots[slot_index].remove(vec_index);
        }
    }

    fn focusing_power(&self) -> usize {
        let mut result: usize = 0;
        for (index, vec) in self.slots.iter().enumerate() {
            for (lens_index, (_label, lens)) in vec.iter().enumerate() {
                result += (index + 1) * (lens_index + 1) * (*lens as usize);
            }
        }
        result
    }

    #[allow(dead_code)]
    fn print(&self) {
        for (i, vec) in self.slots.iter().enumerate() {
            if !vec.is_empty() {
                println!("Box {}: {:?}", i, vec);
            }
        }
        println!();
    }
}

pub fn day15_part_1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

pub fn day15_part_2(input: &str) -> usize {
    let mut map = HashMap::new();

    for command in input.split(',') {
        if command.ends_with('-') {
            map.remove(&command[0..command.len() - 1]);
        } else {
            let mut pieces = command.split('=');
            let label = pieces.next().expect("Label should exist.");
            let value: u8 = pieces
                .next()
                .expect("Value should exist.")
                .parse()
                .expect("Value should be a valid u8.");
            map.insert(label, value);
        }
    }

    map.focusing_power()
}

#[cfg(test)]
mod tests {
    use crate::day15::{day15_part_1, day15_part_2, hash};

    #[test]
    pub fn part1_example() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(day15_part_1("HASH"), 52);
        assert_eq!(
            day15_part_1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day15_part_2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145
        );
    }
}
