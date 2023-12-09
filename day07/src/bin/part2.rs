use std::{cmp::Ordering, collections::HashMap, iter::zip};

use nom::{
    character::complete::{alphanumeric1, digit1, newline, space1},
    combinator::{all_consuming, map, map_res, opt},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

use rayon::prelude::*;

fn main() {
    let data = include_str!("data");
    let result = process(data);
    dbg!(result);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Card {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::N9,
            '8' => Card::N8,
            '7' => Card::N7,
            '6' => Card::N6,
            '5' => Card::N5,
            '4' => Card::N4,
            '3' => Card::N3,
            '2' => Card::N2,
            s => panic!("Not a Card: {s}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand([Card; 5]);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Hand {
    fn generate_possible_hands(hand: &Hand) -> Vec<Hand> {
        // FIXME: Really inefficient
        if !hand.0.contains(&Card::J) {
            return vec![*hand];
        }
        let inx = hand.0.iter().position(|c| *c == Card::J).unwrap();
        use Card::*;
        let mut possible_hands = Vec::new();
        for card in [A, K, Q, T, N9, N8, N7, N6, N5, N4, N3, N2] {
            let mut hand = *hand;
            hand.0[inx] = card;
            possible_hands.extend(Hand::generate_possible_hands(&hand))
        }
        possible_hands
    }

    fn get_rank(&self) -> Rank {
        let map = self.0.iter().fold(HashMap::new(), |mut map, item| {
            map.entry(item).and_modify(|cnt| *cnt += 1).or_insert(1);
            map
        });
        match map.keys().len() {
            5 => Rank::HighCard,
            4 => Rank::OnePair,
            3 => match *map.values().max().unwrap() {
                3 => Rank::ThreeKind,
                2 => Rank::TwoPair,
                _ => panic!("Impossible!"),
            },
            2 => match *map.values().max().unwrap() {
                4 => Rank::FourKind,
                3 => Rank::FullHouse,
                _ => panic!("Impossible!"),
            },
            1 => Rank::FiveKind,
            _ => panic!("Impossible!"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut p_self = Hand::generate_possible_hands(self);
        p_self.sort();
        let mut p_other = Hand::generate_possible_hands(other);
        p_other.sort();

        let (rank1, rank2) = (
            p_self.last().unwrap().get_rank(),
            p_other.last().unwrap().get_rank(),
        );
        if rank1 == rank2 {
            for (card1, card2) in zip(self.0.iter(), other.0.iter()) {
                if card1 != card2 {
                    return card1.cmp(card2);
                }
            }
            return Ordering::Equal;
        }
        match rank1 > rank2 {
            true => Ordering::Greater,
            false => Ordering::Less,
        }
    }
}

fn parse_line(data: &str) -> IResult<&str, (Hand, usize)> {
    terminated(
        separated_pair(
            map(alphanumeric1, |s: &str| {
                assert_eq!(s.len(), 5);
                let mut hand = [Card::A; 5];
                s.chars()
                    .map(|c| c.into())
                    .enumerate()
                    .for_each(|(inx, card)| hand[inx] = card);
                Hand(hand)
            }),
            space1,
            map_res(digit1, |n: &str| n.parse::<usize>()),
        ),
        opt(newline),
    )(data)
}

fn parse(data: &str) -> Vec<(Hand, usize)> {
    all_consuming(many1(parse_line))(data).unwrap().1
}

fn process(data: &str) -> usize {
    let mut game = parse(data);
    game.par_sort();
    game.iter()
        .enumerate()
        .inspect(|(rank, (hand, bid))| {
            println!("{}: {:?} - {} => {}", rank + 1, hand, bid, (rank + 1) * bid)
        })
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum()
}

mod tests {
    #[test]
    fn part2() {
        assert_eq!(
            super::process(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            5905
        )
    }
    #[test]
    fn part2_custom() {
        assert_eq!(
            super::process(
                "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"
            ),
            6839
        )
    }
}
