fn main() {
    let data = include_str!("part1.data"); // Same data as part one
    let result = process(data);
    dbg!(result);
}

fn process(data: &str) -> u32 {
    let map = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut sum = 0;
    for line in data.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut line = line.to_owned();
        let mut max_inx = (0, "");
        let mut min_inx = (line.len(), "");
        for (str, val) in map {
            if let Some(inx) = line.rfind(str) {
                if inx > max_inx.0 {
                    max_inx = (inx, val);
                }
            }
            if let Some(inx) = line.find(str) {
                if inx < min_inx.0 {
                    min_inx = (inx, val);
                }
            }
        }
        line.insert_str(max_inx.0, max_inx.1);
        line.insert_str(min_inx.0, min_inx.1);

        let num = format!(
            "{}{}",
            line.chars().find(char::is_ascii_digit).unwrap(),
            line.chars().rev().find(char::is_ascii_digit).unwrap()
        )
        .parse::<u32>()
        .unwrap();
        sum += num;
    }
    sum
}

mod tests {
    #[test]
    fn test() {
        assert_eq!(
            super::process(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        )
    }
}
