use std::{cell::Cell, num::ParseIntError, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, none_of},
    combinator::{all_consuming, map, map_res, opt},
    multi::{many1, many1_count},
    sequence::terminated,
    IResult,
};

fn main() {
    let data = include_str!("data");
    let result = process(data);
    dbg!(result);
}

#[derive(Clone, Debug)]
enum Field {
    Empty,
    Value(usize, Rc<Cell<bool>>),
    Symbol(char),
}

fn parse_line(line: &str) -> IResult<&str, Vec<(Field, usize)>> {
    terminated(
        many1(alt((
            map(many1_count(tag(".")), |n: usize| (Field::Empty, n)),
            map_res(digit1, |s: &str| -> Result<(Field, usize), ParseIntError> {
                let n = s.parse::<usize>()?;
                Ok((Field::Value(n, Rc::new(Cell::new(false))), s.len()))
            }),
            map(none_of("\n"), |c: char| (Field::Symbol(c), 1)),
        ))),
        opt(newline),
    )(line)
}

fn parse(data: &str) -> Vec<Vec<(Field, usize)>> {
    all_consuming(many1(parse_line))(data)
        .expect("Couldn't Parse")
        .1
}
fn generate_circle(row_inx: usize, col_inx: usize) -> [(isize, isize); 8] {
    let (row_inx, col_inx) = (row_inx as isize, col_inx as isize);
    [
        (row_inx - 1, col_inx - 1),
        (row_inx - 1, col_inx),
        (row_inx - 1, col_inx + 1),
        (row_inx, col_inx - 1),
        (row_inx, col_inx + 1),
        (row_inx + 1, col_inx - 1),
        (row_inx + 1, col_inx),
        (row_inx + 1, col_inx + 1),
    ]
}

fn process(data: &str) -> usize {
    let board = parse(data)
        .into_iter()
        .map(|row| {
            row.iter()
                .flat_map(|(field, len)| match field {
                    Field::Empty => vec![Field::Empty; *len],
                    Field::Value(num, cell) => {
                        vec![Field::Value(*num, cell.clone()); *len]
                    }
                    Field::Symbol(s) => vec![Field::Symbol(*s); *len],
                })
                .collect::<Vec<Field>>()
        })
        .collect::<Vec<Vec<Field>>>();

    let mut sum = 0;
    for (row_inx, row) in board.iter().enumerate() {
        for (col_inx, field) in row.iter().enumerate() {
            if let Field::Symbol(_s) = field {
                let adjacent_values: Vec<(&usize, &Rc<Cell<bool>>)> =
                    generate_circle(row_inx, col_inx)
                        .iter()
                        .filter(|(row_inx, col_inx)| *row_inx >= 0 && *col_inx >= 0)
                        .map(|(row_inx, col_inx)| (*row_inx as usize, *col_inx as usize))
                        .filter_map(|(row_inx, col_inx)| board.get(row_inx)?.get(col_inx))
                        .filter_map(|field| {
                            if let Field::Value(value, tagged) = field {
                                return Some((value, tagged));
                            }
                            None
                        })
                        .collect();
                let (cnt, ratio) =
                    adjacent_values
                        .iter()
                        .fold((0usize, 1usize), |mut cnt, (val, tag)| {
                            if !tag.get() {
                                cnt.0 += 1;
                                cnt.1 *= **val;
                                tag.set(true);
                            }
                            cnt
                        });
                adjacent_values.iter().for_each(|(_, cell)| cell.set(false));
                if cnt == 2 {
                    sum += ratio
                }
            }
        }
    }
    sum
}

mod tests {
    #[test]
    fn part2() {
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
            467835
        )
    }
}
