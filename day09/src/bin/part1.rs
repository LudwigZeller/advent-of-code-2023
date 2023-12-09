#![feature(iter_map_windows)]
use nom::{
    character::complete::{char, digit1, newline, space1},
    combinator::{all_consuming, map, map_res, opt},
    multi::{many1, separated_list1},
    sequence::{pair, terminated},
    IResult, Parser,
};

fn main() {
    let data = include_str!("data");
    let result = process(data);
    dbg!(result);
}

fn parse_line(data: &str) -> IResult<&str, Vec<isize>> {
    terminated(
        separated_list1(
            space1,
            map(
                pair(opt(char('-')), map_res(digit1, |num: &str| num.parse())),
                |(sign, num): (Option<char>, isize)| match sign {
                    Some(_) => -num,
                    None => num,
                },
            ),
        ),
        opt(newline),
    )(data)
}

fn parse(data: &str) -> Vec<Vec<isize>> {
    all_consuming(many1(parse_line))(data).unwrap().1
}

fn generate_difference_series(series: &[isize]) -> Vec<isize> {
    series
        .iter()
        .map_windows(|[x, y]| **y - **x)
        .collect::<Vec<isize>>()
}

fn process(data: &str) -> isize {
    parse(data)
        .into_iter()
        .map(|series| {
            let mut differences = vec![series];
            while !differences.last().unwrap().iter().all(|x| *x == 0) {
                differences.push(generate_difference_series(differences.last().unwrap()))
            }
            differences
                .into_iter()
                .rev()
                .fold(0isize, |difference, series| {
                    difference + series.last().unwrap()
                })
        })
        .sum::<isize>()
}

mod tests {
    #[test]
    fn part1() {
        assert_eq!(
            super::process(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            114
        )
    }
}
