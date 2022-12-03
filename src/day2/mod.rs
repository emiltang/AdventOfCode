use std::cmp::Ordering;

use super::util::read;

#[derive(PartialEq, Eq)]
enum Gesture {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Gesture {
    fn score(&self) -> i32 {
        return match self {
            Gesture::Rock => 1,
            Gesture::Paper => 2,
            Gesture::Scissor => 3,
        };
    }

    pub fn parse(s: &str) -> Gesture {
        return match s {
            "X" | "A" => Gesture::Rock,
            "Y" | "B" => Gesture::Paper,
            "Z" | "C" => Gesture::Scissor,
            _ => panic!("Invalid input"),
        };
    }
}

impl Ord for Gesture {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Gesture::Rock, Gesture::Rock) => Ordering::Equal,
            (Gesture::Paper, Gesture::Paper) => Ordering::Equal,
            (Gesture::Scissor, Gesture::Scissor) => Ordering::Equal,
            (Gesture::Scissor, Gesture::Rock) => Ordering::Greater,
            (Gesture::Rock, Gesture::Paper) => Ordering::Greater,
            (Gesture::Paper, Gesture::Scissor) => Ordering::Greater,
            (Gesture::Rock, Gesture::Scissor) => Ordering::Less,
            (Gesture::Paper, Gesture::Rock) => Ordering::Less,
            (Gesture::Scissor, Gesture::Paper) => Ordering::Less,
        }
    }
}

impl PartialOrd for Gesture {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

enum Outcome {
    Win,
    Draw,
    Loose,
}

impl Outcome {
    fn score(&self) -> i32 {
        return match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loose => 0,
        };
    }

    fn parse(s: &str) -> Outcome {
        return match s {
            "X" | "A" => Outcome::Loose,
            "Y" | "B" => Outcome::Draw,
            "Z" | "C" => Outcome::Win,
            _ => panic!("Invalid input"),
        };
    }
}

struct RoundOne {
    elf: Gesture,
    player: Gesture,
}

impl RoundOne {
    fn play(&self) -> i32 {
        return match self.elf.cmp(&self.player) {
            Ordering::Equal => self.player.score() + Outcome::Draw.score(),
            Ordering::Greater => self.player.score() + Outcome::Win.score(),
            Ordering::Less => self.player.score() + Outcome::Loose.score(),
        };
    }

    fn parse(lines: &Vec<String>) -> Vec<RoundOne> {
        let mut commands: Vec<RoundOne> = Vec::new();

        for line in lines {
            let elem: Vec<&str> = line.split_ascii_whitespace().collect();
            let tuple = RoundOne {
                elf: Gesture::parse(elem[0]),
                player: Gesture::parse(elem[1]),
            };
            commands.push(tuple)
        }
        return commands;
    }
}

struct RoundTwo {
    elf: Gesture,
    outcome: Outcome,
}

impl RoundTwo {
    fn choose_counter(&self) -> Gesture {
        return match (&self.elf, &self.outcome) {
            (Gesture::Rock, Outcome::Win) =>  Gesture::Paper,
            (Gesture::Rock, Outcome::Draw) => Gesture::Rock,
            (Gesture::Rock, Outcome::Loose) => Gesture::Scissor,
            (Gesture::Paper, Outcome::Win) => Gesture::Scissor,
            (Gesture::Paper, Outcome::Draw) => Gesture::Paper,
            (Gesture::Paper, Outcome::Loose) => Gesture::Rock,
            (Gesture::Scissor, Outcome::Win) => Gesture::Rock,
            (Gesture::Scissor, Outcome::Draw) => Gesture::Scissor,
            (Gesture::Scissor, Outcome::Loose) => Gesture::Paper,
        };
    }

    fn play(&self) -> i32 {
        return self.choose_counter().score() + self.outcome.score();
    }

    fn parse(lines: &Vec<String>) -> Vec<RoundTwo> {
        return lines
            .iter()
            .map(|line| {
                let elem: Vec<&str> = line.split_ascii_whitespace().collect();
                RoundTwo {
                    elf: Gesture::parse(elem[0]),
                    outcome: Outcome::parse(elem[1]),
                }
            })
            .collect();
    }
}

fn calc_score(file_name: &str) -> i32 {
    let lines = read(file_name).expect("File read error");
    let rounds = RoundOne::parse(&lines);
    return rounds.iter().map(|r| r.play()).sum();
}

fn part_two(file_name: &str) -> i32 {
    let lines = read(file_name).expect("File read error");
    let rounds = RoundTwo::parse(&lines);
    return rounds.iter().map(|r| r.play()).sum();
}

#[cfg(test)]
mod tests {
    use super::{calc_score, part_two};

    #[test]
    fn part_1_small_set() {
        assert_eq!(15, calc_score("src/day2/test"))
    }

    #[test]
    fn part_1_large_set() {
        assert_eq!(15422, calc_score("src/day2/input"))
    }

    #[test]
    fn part_2_small_set() {
        assert_eq!(12, part_two("src/day2/test"))
    }

    #[test]
    fn part_2_large_set() {
        assert_eq!(15442, part_two("src/day2/input"))
    }
}
