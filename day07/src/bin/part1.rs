use std::{
    char::ParseCharError, cmp::Ordering, collections::HashMap, iter::zip, string::ParseError,
};

use nom::{
    character::complete::{alpha1, alphanumeric1, digit1, newline, space1},
    combinator::{all_consuming, map, map_res, opt},
    multi::many1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn main() {
    let data = include_str!("data");
    let result = process(data);
    dbg!(result);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Kind {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
}

impl From<char> for Kind {
    fn from(value: char) -> Self {
        match value {
            'A' => Kind::A,
            'K' => Kind::K,
            'Q' => Kind::Q,
            'J' => Kind::J,
            'T' => Kind::T,
            '9' => Kind::N9,
            '8' => Kind::N8,
            '7' => Kind::N7,
            '6' => Kind::N6,
            '5' => Kind::N5,
            '4' => Kind::N4,
            '3' => Kind::N3,
            '2' => Kind::N2,
            s => panic!("Not a Kind: {s}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand([Kind; 5]);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
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
        let (rank1, rank2) = (self.get_rank(), other.get_rank());
        if rank1 == rank2 {
            for (card1, card2) in zip(self.0.iter(), other.0.iter()) {
                if card1 != card2 {
                    return Some(card1.cmp(card2));
                }
            }
            return Some(Ordering::Equal);
        }
        match rank1 > rank2 {
            true => Some(Ordering::Greater),
            false => Some(Ordering::Less),
        }
    }
}

fn parse_line(data: &str) -> IResult<&str, (Hand, usize)> {
    terminated(
        separated_pair(
            map(alphanumeric1, |s: &str| {
                assert_eq!(s.len(), 5);
                let mut hand = [Kind::A; 5];
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
    game.sort_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());
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
    fn part1() {
        assert_eq!(
            super::process(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            6440
        )
    }
}
