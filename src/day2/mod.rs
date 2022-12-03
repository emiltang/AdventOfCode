use std::cmp::Ordering;

use super::util::read;

#[derive(PartialEq, Eq)]
enum Command {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Command {
    fn score(&self) -> i32 {
        return match self {
            Command::Rock => 1,
            Command::Paper => 2,
            Command::Scissor => 3,
        };
    }
}

impl Ord for Command {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Command::Rock, Command::Rock) => Ordering::Equal,
            (Command::Paper, Command::Paper) => Ordering::Equal,
            (Command::Scissor, Command::Scissor) => Ordering::Equal,
            (Command::Scissor, Command::Rock) => Ordering::Greater,
            (Command::Rock, Command::Paper) => Ordering::Greater,
            (Command::Paper, Command::Scissor) => Ordering::Greater,
            (Command::Rock, Command::Scissor) => Ordering::Less,
            (Command::Paper, Command::Rock) => Ordering::Less,
            (Command::Scissor, Command::Paper) => Ordering::Less,
        }
    }
}

impl PartialOrd for Command {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Round {
    elf: Command,
    player: Command,
}

impl Round {
    fn play(&self) -> i32 {
        return match self.elf.cmp(&self.player) {
            Ordering::Equal => self.player.score() + 3,
            Ordering::Greater => self.player.score() + 6,
            Ordering::Less => self.player.score(),
        };
    }
}

fn parse_command(s: &str) -> Command {
    return match s {
        "X" | "A" => Command::Rock,
        "Y" | "B" => Command::Paper,
        "Z" | "C" => Command::Scissor,
        _ => panic!("Invalid input"),
    };
}

fn parse(lines: &Vec<String>) -> Vec<Round> {
    let mut commands: Vec<Round> = Vec::new();

    for line in lines {
        let elem: Vec<&str> = line.split_ascii_whitespace().collect();
        let tuple = Round {
            elf: parse_command(elem[0]),
            player: parse_command(elem[1]),
        };
        commands.push(tuple)
    }
    return commands;
}

fn calc_score(file_name: &str) -> i32 {
    let lines = read(file_name).expect("File read error");
    let commands = parse(&lines);
    return commands.iter().map(|r| r.play()).sum();
}

#[cfg(test)]
mod tests {
    use super::calc_score;

    #[test]
    fn part_1_small_set() {
        assert_eq!(15, calc_score("src/day2/test"))
    }

    #[test]
    fn part_1_large_set() {
        assert_eq!(15422, calc_score("src/day2/input"))
    }
}
