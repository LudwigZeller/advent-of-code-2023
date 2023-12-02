fn main() {
    let data = include_str!("part1.data");
    let result = process(data);
    dbg!(result);
}

fn process(data: &str) -> u32 {
    data.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            format!(
                "{}{}",
                line.chars().find(char::is_ascii_digit).unwrap(),
                line.chars().rev().find(char::is_ascii_digit).unwrap()
            )
            .parse::<u32>()
            .unwrap()
        })
        .sum()
}

mod tests {
    #[test]
    fn test() {
        assert_eq!(
            super::process(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        )
    }
}
