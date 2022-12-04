use advent_of_code_2022::read_input;
#[derive(PartialEq, Eq, Clone, Copy)]
enum Object {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Play {
    opponent: Object,
    you: Object,
}
impl Play {
    pub fn parse_part1(line: &str) -> Self {
        let split = line.split_ascii_whitespace().collect::<Vec<_>>();
        let opponent = match split[0] {
            "A" => Object::Rock,
            "B" => Object::Paper,
            "C" => Object::Scissors,
            _ => panic!("invalid opponent object, must be A, B and C"),
        };
        let you = match split[1] {
            "X" => Object::Rock,
            "Y" => Object::Paper,
            "Z" => Object::Scissors,
            _ => panic!("invalid opponent object, must be X, Y and Z"),
        };
        Self { opponent, you }
    }
    pub fn get_score(&self) -> usize {
        let mut score = 0usize;
        score += match self.you {
            Object::Rock => 1,
            Object::Paper => 2,
            Object::Scissors => 3,
        };

        match (self.you, self.opponent) {
            (you, opponent) if you == opponent => {
                score += 3;
            }
            (Object::Paper, Object::Rock) => score += 6,
            (Object::Paper, Object::Scissors) => {}
            (Object::Scissors, Object::Rock) => {}
            (Object::Scissors, Object::Paper) => score += 6,
            (Object::Rock, Object::Paper) => {}
            (Object::Rock, Object::Scissors) => score += 6,
            _ => unreachable!(),
        };
        score
    }

    pub fn parse_part2(line: &str) -> Self {
        let split = line.split_ascii_whitespace().collect::<Vec<_>>();
        let opponent = match split[0] {
            "A" => Object::Rock,
            "B" => Object::Paper,
            "C" => Object::Scissors,
            _ => panic!("invalid opponent object, must be A, B and C"),
        };
        let you = match (split[1], opponent) {
            ("Z", Object::Paper) => Object::Scissors,
            ("Z", Object::Rock) => Object::Paper,
            ("Z", Object::Scissors) => Object::Rock,
            ("Y", opponent) => opponent,
            ("X", Object::Paper) => Object::Rock,
            ("X", Object::Rock) => Object::Scissors,
            ("X", Object::Scissors) => Object::Paper,
            _ => panic!("invalid opponent object, must be X, Y and Z"),
        };
        Self { opponent, you }
    }
}
pub fn part1(input: &str) {
    let score = input
        .split('\n')
        .map(Play::parse_part1)
        .map(|p| Play::get_score(&p))
        .sum::<usize>();
    println!("Part 1: {score}");
}
pub fn part2(input: &str) {
    let score = input
        .split('\n')
        .map(Play::parse_part2)
        .map(|p| Play::get_score(&p))
        .sum::<usize>();
    println!("Part 2: {score}");
}
fn main() {
    let input = read_input(2, true);
    part1(&input);
    part2(&input);
}
