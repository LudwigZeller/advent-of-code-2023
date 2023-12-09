use std::iter::zip;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space1},
    combinator::{all_consuming, map_res, opt},
    multi::separated_list1,
    sequence::{delimited, pair, tuple},
    IResult,
};

fn main() {
    let data = include_str!("data");
    let result = process(data);
    dbg!(result);
}
fn parse_line(data: &str) -> IResult<&str, usize> {
    delimited(
        tuple((alpha1, tag(":"), space1)),
        map_res(separated_list1(space1, digit1), |s: Vec<&str>| {
            s.into_iter().collect::<String>().parse()
        }),
        opt(newline),
    )(data)
}

fn parse(data: &str) -> (usize, usize) {
    all_consuming(pair(parse_line, parse_line))(data).unwrap().1
}

fn calculate_margin(time: usize, record: usize) -> usize {
    /*
     * d(t) = t * (t_r - t)
     * d(t) = t * t_r - t^2
     *
     * d_min < d(t)
     * d_min < -t^2 + t * t_r
     * 0 < -t^2 + t * t_r - d_min
     * t_1 = 1/2 * (t_r - sqrt(t_r^2 - 4 * d_min))
     * t_2 = 1/2 * (t_r + sqrt(t_r^2 - 4 * d_min))
     *
     * m = t_2 - t_1
     * m = 0.5*t_r + 0.5*sqrt(...) - (0.5*t_r - 0.5*sqrt(...))
     * m = sqrt(...)
     */
    let (time, record) = (time as f64, record as f64);
    let sqrt = (time * time - 4. * record).sqrt();
    let low = 0.5 * time - 0.5 * sqrt;
    let high = 0.5 * time + 0.5 * sqrt;

    // I HATE FLOATING POINT NUMBERS
    ((high - 0.001).floor() - (low + 0.001).ceil()) as usize + 1
}

fn process(data: &str) -> usize {
    let (time, record) = parse(data);
    calculate_margin(time, record)
}

mod tests {
    #[test]
    fn part1() {
        assert_eq!(
            super::process(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            71503
        )
    }
}
