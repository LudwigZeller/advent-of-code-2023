use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    combinator::{all_consuming, map, opt},
    multi::many1,
    sequence::{delimited, pair, separated_pair, terminated},
    IResult,
};

fn main() {
    let data = include_str!("data");
    let result = process(data);
    dbg!(result);
}

#[derive(Debug)]
enum Direction {
    R,
    L,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Node<'a>(&'a str);

fn parse_direction(data: &str) -> IResult<&str, Vec<Direction>> {
    terminated(
        many1(map(one_of("RL"), |c: char| match c {
            'R' => Direction::R,
            'L' => Direction::L,
            _ => panic!("Unexpected Char in Direction"),
        })),
        tag("\n\n"),
    )(data)
}

fn parse_node(data: &str) -> IResult<&str, (Node, (Node, Node))> {
    terminated(
        separated_pair(
            map(alpha1, |s: &str| Node(s)),
            tag(" = "),
            delimited(
                tag("("),
                map(
                    separated_pair(alpha1, tag(", "), alpha1),
                    |(s1, s2): (&str, &str)| (Node(s1), Node(s2)),
                ),
                tag(")"),
            ),
        ),
        opt(line_ending),
    )(data)
}
fn parse(data: &str) -> (Vec<Direction>, Vec<(Node, (Node, Node))>) {
    all_consuming(pair(parse_direction, many1(parse_node)))(data)
        .unwrap()
        .1
}

fn process(data: &str) -> usize {
    let (directions, nodes) = parse(data);
    let nodes = nodes
        .into_iter()
        .fold(HashMap::new(), |mut map, (node, entry)| {
            map.insert(node, entry);
            map
        });

    let mut current_node = Node("AAA");
    let mut steps = 0;

    while current_node != Node("ZZZ") {
        for dir in &directions {
            let (left, right) = nodes.get(&current_node).unwrap();
            match dir {
                Direction::R => current_node = *right,
                Direction::L => current_node = *left,
            }
            steps += 1;
        }
    }
    steps
}

mod tests {
    #[test]
    fn part1_1() {
        assert_eq!(
            super::process(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            2
        )
    }

    #[test]
    fn part1_2() {
        assert_eq!(
            super::process(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        )
    }
}
