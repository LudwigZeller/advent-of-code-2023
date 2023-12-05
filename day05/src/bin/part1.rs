use nom::*;
fn main() {
    let data = include_str!("data");
    let result = process(data);
    dbg!(result);
}

fn parse_file(data: &str) -> IResult<&str, &str> {
    todo!()
}

fn process(data: &str) -> usize {
    todo!()
}

mod tests {
    #[test]
    fn part1() {
        assert_eq!(
            super::process(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            4361
        )
    }
}
