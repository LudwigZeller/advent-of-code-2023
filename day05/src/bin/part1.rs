use nom::{
    bytes::complete::{tag, take_until1},
    multi::many0,
    sequence::terminated,
    IResult,
};

fn main() {
    let data = include_str!("data");
    let result = process(data);
    dbg!(result);
}

struct Translator<'a> {
    id: (&'a str, &'a str),
    input: Vec<(usize, usize)>,
}

impl<'a> From<&'a str> for Translator<'a> {
    fn from(value: &'a str) -> Self {
        todo!()
    }
}

fn parse_multiline_block(data: &str) -> IResult<&str, &str> {
    terminated(take_until1("\n\n"), tag("\n\n"))(data)
}

fn parse_all_multiline_blocks(data: &str) -> IResult<&str, Vec<&str>> {
    many0(parse_multiline_block)(data)
}

fn process(data: &str) -> usize {
    dbg!(parse_all_multiline_blocks(data).unwrap().1);
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
