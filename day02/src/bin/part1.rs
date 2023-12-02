fn main() {
    let data = include_str!("part1.data");
    let result = process(data);
    dbg!(result);
}

#[derive(Debug)]
struct Bag {
    id: u32,
    cubes: Vec<(u32, u32, u32)>,
}

fn parse(line: &str) -> Bag {
    let id = line[5..line.find(':').unwrap()].parse().unwrap();
    let line = &line[line.find(':').unwrap() + 1..];
    let cubes = line
        .split(';')
        .map(|game| {
            game.split(',')
                .map(|draw| {
                    let draw = draw[1..].split(' ').collect::<Vec<_>>();
                    let num = draw[0].parse().unwrap();
                    match draw[1] {
                        "red" => (num, 0, 0),
                        "green" => (0, num, 0),
                        "blue" => (0, 0, num),
                        _ => panic!("Unexpected Color"),
                    }
                })
                .reduce(|(r1, g1, b1), (r2, g2, b2)| (r1 + r2, g1 + g2, b1 + b2))
                .unwrap()
        })
        .collect::<Vec<(_, _, _)>>();
    Bag { id, cubes }
}

fn process(data: &str) -> u32 {
    data.split('\n')
        .filter(|line| !line.is_empty())
        .map(parse)
        .filter(|bag| {
            bag.cubes.iter().map(|r| r.0).max().unwrap_or(0u32) <= 12
                && bag.cubes.iter().map(|g| g.1).max().unwrap_or(0u32) <= 13
                && bag.cubes.iter().map(|b| b.2).max().unwrap_or(0u32) <= 14
        })
        .map(|bag| bag.id)
        .sum()
}

mod tests {
    #[test]
    fn part1() {
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
