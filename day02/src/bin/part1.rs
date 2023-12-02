fn main() {
    let data = include_str!("part1.data");
    let result = process(data);
    dbg!(result);
}
struct Bag {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

fn parse(line: &str) -> Bag {
    todo!()
}

fn process(data: &str) -> u32 {
    data.split('\n')
        .filter(|line| !line.is_empty())
        .map(parse)
        .filter(|bag| bag.red <= 12 && bag.green <= 13 && bag.blue <= 14)
        .map(|bag| bag.id)
        .sum()
}

mod tests {
    #[test]
    fn test() {
        assert_eq!(
            super::process(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        )
    }
}
