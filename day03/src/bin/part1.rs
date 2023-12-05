use nom::{IResult, complete::tag,}
fn main() {
    let data = include_str!("data");
    let result = process(data);
    dbg!(result);
}

enum Field {
    Empty,
    Value(usize),
    Cog(char),
}

fn parse(line: &str) -> Vec<Field> {
    todo!()
    //     let mut vec = Vec::with_capacity(line.len());
    //     for (inx, char) in line.chars().enumerate() {
    //         vec.push(match char {
    //             '.' => Field::Empty,
    //             char if char.is_ascii_digit() => {
    //                 todo!()
    //             }
    //             char => Field::Cog(char),
    //         });
    //     }
    //     vec
}

fn process(data: &str) -> usize {
    let a = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(parse)
        .collect::<Vec<Vec<Field>>>();
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
