fn main() {
    let data = include_str!("part1.data");
    let result = process(data);
    dbg!(result);
}

#[derive(Debug)]
struct Card {
    id: usize,
    win: Vec<usize>,
    got: Vec<usize>,
}
impl Card {
    fn wins(&self) -> usize {
        self.got.iter().filter(|num| self.win.contains(num)).count()
    }
}

fn parse(line: &str) -> Card {
    let id: usize = line[line.find(|c: char| c.is_ascii_digit()).unwrap()..line.find(':').unwrap()]
        .parse()
        .unwrap();
    let line = &line[line.find(':').unwrap() + 1..];

    let nums = line
        .split('|')
        .map(|nums| {
            nums.split(' ')
                .filter(|str| !str.is_empty())
                .map(|str| str.trim())
                .map(|str| str.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    Card {
        id,
        win: nums[0].clone(),
        got: nums[1].clone(),
    }
}

fn process(data: &str) -> usize {
    let mut cards = data.split('\n').filter(|line| !line.is_empty()).map(parse);
    /*
    .try_fold(
        (0, std::collections::VecDeque::<usize>::new()),
        |(mut sum, mut queue), card| {
            if queue.is_empty() {
                return std::ops::ControlFlow::Break((sum, queue));
            }
            std::ops::ControlFlow::Continue((sum, queue))
        },
    )
    .break_value()
    .unwrap()
    .0
    .collect::<Vec<Card>>()
    */

    let first = cards.next().unwrap().wins();
    let mut counts = vec![1; cards.clone().count() + 1];
    for inx in 1..first + 1 {
        counts[inx] += 1;
    }
    dbg!(&counts);
    for card in cards {
        for inx in card.id..card.id + card.wins() {
            match counts.get(inx) {
                Some(_) => {
                    counts[inx] += counts[card.id - 1];
                }
                None => {
                    counts.push(counts[card.id - 1]);
                }
            };
        }
        dbg!(card.id, card.wins(), &counts);
    }
    counts.iter().sum()
}

mod tests {
    #[test]
    fn part2() {
        assert_eq!(
            super::process(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            30
        )
    }
}
