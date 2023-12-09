use dashmap::DashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, one_of},
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
            map(alphanumeric1, |s: &str| Node(s)),
            tag(" = "),
            delimited(
                tag("("),
                map(
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
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
    let mut current_nodes: Vec<Node> = nodes
        .iter()
        .map(|(node, _)| *node)
        .filter(|node| node.0.ends_with('A'))
        .collect();
    let nodes = nodes
        .into_iter()
        .fold(DashMap::new(), |map, (node, entry)| {
            map.insert(node, entry);
            map
        });

    let mut steps = 0;
    let mut last = 1;

    // Input Data is cyclic where the distance start => end == end => start
    let mut start_nodes = current_nodes;
    start_nodes
        .iter_mut()
        .map(|mut current_node| {
            let mut steps = 0usize;
            while !current_node.0.ends_with('Z') {
                for dir in &directions {
                    let (left, right) = *nodes.get(current_node).unwrap();
                    match dir {
                        Direction::R => *current_node = right,
                        Direction::L => *current_node = left,
                    }

                    steps += 1;
                }
            }
            steps
        })
        .reduce(num::integer::lcm)
        .unwrap()

    // Correct but extremely slow
    // while !current_nodes.iter().all(|node| node.0.ends_with('Z')) {
    //     for dir in &directions {
    //         for current_node in current_nodes.iter_mut() {
    //             let (left, right) = *nodes.get(current_node).unwrap();
    //             match dir {
    //                 Direction::R => *current_node = right,
    //                 Direction::L => *current_node = left,
    //             }
    //         }
    //         steps += 1;
    //     }
    //     if steps / last > 10 {
    //         last = steps;
    //         println!("{steps}");
    //     }
    // }
    // steps
}

mod tests {
    #[test]
    fn part2() {
        assert_eq!(
            super::process(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        )
    }
}
