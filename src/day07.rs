use std::{cmp::Ordering, collections::HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
enum CardKind {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}

impl CardKind {
    const VALUES: [CardKind; 14] = [
        CardKind::A,
        CardKind::K,
        CardKind::Q,
        CardKind::J,
        CardKind::T,
        CardKind::Nine,
        CardKind::Eight,
        CardKind::Seven,
        CardKind::Six,
        CardKind::Five,
        CardKind::Four,
        CardKind::Three,
        CardKind::Two,
        CardKind::One,
    ];

    fn from(input: char) -> CardKind {
        match input {
            'A' => CardKind::A,
            'K' => CardKind::K,
            'Q' => CardKind::Q,
            'J' => CardKind::J,
            'T' => CardKind::T,
            '9' => CardKind::Nine,
            '8' => CardKind::Eight,
            '7' => CardKind::Seven,
            '6' => CardKind::Six,
            '5' => CardKind::Five,
            '4' => CardKind::Four,
            '3' => CardKind::Three,
            '2' => CardKind::Two,
            '1' => CardKind::One,
            _ => panic!("Received unexpected char for CardType: {}", input),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandKind {
    fn from(cards: &[CardKind; 5]) -> HandKind {
        let mut counts: HashMap<CardKind, usize> = HashMap::new();

        for card in cards {
            let entry = counts.entry(*card).or_insert(0);
            *entry += 1;
        }

        let mut counts: Vec<usize> = counts.into_values().collect();
        counts.sort_by(|a, b| b.cmp(a));
        if counts.len() == 1 {
            return HandKind::FiveOfAKind;
        }

        if counts[0] == 4 && counts[1] == 1 {
            return HandKind::FourOfAKind;
        }

        if counts[0] == 3 && counts[1] == 2 {
            return HandKind::FullHouse;
        }

        if counts[0] == 3 && counts[1] == 1 {
            return HandKind::ThreeOfAKind;
        }

        if counts.len() == 3 && counts[0] == 2 && counts[1] == 2 {
            return HandKind::TwoPair;
        }

        if counts.len() == 4 {
            return HandKind::OnePair;
        }

        HandKind::HighCard
    }

    fn from_with_jokers(cards: &[CardKind; 5]) -> HandKind {
        let mut best = HandKind::from(cards);
        for card in CardKind::VALUES {
            if card == CardKind::J {
                continue;
            }

            let alternate_hand = cards.map(|existing| {
                if existing == CardKind::J {
                    card
                } else {
                    existing
                }
            });

            let kind = HandKind::from(&alternate_hand);
            if kind < best {
                best = kind;
            }
        }
        best
    }
}

#[derive(Debug)]
struct HandAndBid {
    hand: [CardKind; 5],
    kind: HandKind,
    bid: i64,
}

impl HandAndBid {
    fn from(line: &str, j_is_joker: bool) -> HandAndBid {
        let split: Vec<_> = line.split(' ').collect();
        let mut hand: [CardKind; 5] = [
            CardKind::A,
            CardKind::A,
            CardKind::A,
            CardKind::A,
            CardKind::A,
        ];
        for (index, char) in split[0].chars().enumerate() {
            hand[index] = CardKind::from(char);
        }

        let kind = if j_is_joker {
            HandKind::from_with_jokers(&hand)
        } else {
            HandKind::from(&hand)
        };
        let bid: i64 = split[1]
            .parse()
            .expect("Expected bid to be a valid number.");

        HandAndBid { hand, kind, bid }
    }
}

pub fn day07_part_1(input: &str) -> i64 {
    let mut hands: Vec<_> = input
        .split('\n')
        .map(|line| HandAndBid::from(line, false))
        .collect();

    hands.sort_by(|a, b| {
        let ordering = a.kind.cmp(&b.kind);
        if ordering != Ordering::Equal {
            return ordering;
        }

        for i in 0..5 {
            let ordering = a.hand[i].cmp(&b.hand[i]);
            if ordering != Ordering::Equal {
                return ordering;
            }
        }

        Ordering::Equal
    });

    let mut result: i64 = 0;
    for (index, hand) in hands.iter().rev().enumerate() {
        let rank = (index + 1) as i64;
        result += rank * hand.bid;
    }

    result
}

pub fn day07_part_2(input: &str) -> i64 {
    let mut hands: Vec<_> = input
        .split('\n')
        .map(|line| HandAndBid::from(line, true))
        .collect();

    hands.sort_by(|a, b| {
        let ordering = a.kind.cmp(&b.kind);
        if ordering != Ordering::Equal {
            return ordering;
        }

        for i in 0..5 {
            if a.hand[i] == CardKind::J && b.hand[i] != CardKind::J {
                return Ordering::Greater;
            }

            if a.hand[i] != CardKind::J && b.hand[i] == CardKind::J {
                return Ordering::Less;
            }

            let ordering = a.hand[i].cmp(&b.hand[i]);
            if ordering != Ordering::Equal {
                return ordering;
            }
        }

        Ordering::Equal
    });

    let mut result: i64 = 0;
    for (index, hand) in hands.iter().rev().enumerate() {
        let rank = (index + 1) as i64;
        result += rank * hand.bid;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day07::{day07_part_1, day07_part_2};

    #[test]
    pub fn part1_example() {
        assert_eq!(
            day07_part_1(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            6440
        );
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(
            day07_part_2(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            5905
        );
    }
}
