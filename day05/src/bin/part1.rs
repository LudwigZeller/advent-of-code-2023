use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{alpha0, digit1, multispace1, newline, space1},
    combinator::{all_consuming, map, map_res, opt},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, separated_pair, terminated},
    IResult,
};

fn main() {
    let data = include_str!("data");
    let result = process(data);
    dbg!(result);
}

#[derive(Debug)]
struct Translator<'a> {
    id: (&'a str, &'a str),
    trans: Vec<Vec<usize>>,
}

impl<'a> Translator<'a> {
    fn translate(from: usize) -> usize {
        todo!()
    }
}

fn parse_translator_id(data: &str) -> IResult<&str, (&str, &str)> {
    terminated(separated_pair(alpha0, tag("-to-"), is_not(":")), tag(":\n"))(data)
}

fn parse_translator_values(data: &str) -> IResult<&str, Vec<usize>> {
    terminated(
        separated_list1(space1, map_res(digit1, |s: &str| s.parse::<usize>())),
        opt(newline),
    )(data)
}

fn parse_translator(data: &str) -> IResult<&str, Translator> {
    map(
        terminated(
            pair(parse_translator_id, many1(parse_translator_values)),
            opt(newline),
        ),
        |(id, trans)| Translator { id, trans },
    )(data)
}

fn parse_seed(data: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        tag("seeds: "),
        separated_list1(space1, map_res(digit1, |s: &str| s.parse())),
        pair(newline, newline),
    )(data)
}

fn parse(data: &str) -> (Vec<usize>, Vec<Translator>) {
    all_consuming(pair(parse_seed, many1(parse_translator)))(data)
        .unwrap()
        .1
}

fn process(data: &str) -> usize {
    dbg!(parse(data));
    todo!()
}

mod tests {
    #[test]
    fn part1() {
        assert_eq!(
            super::process(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            4361
        )
    }
}
