use super::util::read;

#[derive(PartialEq, Eq)]
enum Command {
    Rock,
    Paper,
    Scissor,
}

struct Round {
    elf: Command,
    player: Command,
}

fn match_elf_strategy(s: &str) -> Command {
    return match s {
        "A" => Command::Rock,
        "B" => Command::Paper,
        "C" => Command::Scissor,
        _ => panic!("Invalid input"),
    };
}

fn match_player_strategy(s: &str) -> Command {
    return match s {
        "X" => Command::Rock,
        "Y" => Command::Paper,
        "Z" => Command::Scissor,
        _ => panic!("Invalid input"),
    };
}

fn score(c: &Command) -> i32 {
    return match c {
        Command::Rock => 1,
        Command::Paper => 2,
        Command::Scissor => 3,
        _ => panic!("Invalid input"),
    };
}

fn play(elf: &Command, player: &Command) -> i32 {
    return match (elf, player) {
        _ if elf == player => score(player) + 3,
        (Command::Scissor, Command::Rock) => score(player) + 6,
        (Command::Rock, Command::Paper) => score(player) + 6,
        (Command::Paper, Command::Scissor) => score(player) + 6,
        (Command::Rock, Command::Scissor) => score(player),
        (Command::Paper, Command::Rock) => score(player),
        (Command::Scissor, Command::Paper) => score(player),
        _ => 0,
    };
}

fn parse(lines: &Vec<String>) -> Vec<Round> {
    let mut commands: Vec<Round> = Vec::new();

    for line in lines {
        let elem: Vec<&str> = line.split_ascii_whitespace().collect();
        let tuple = Round {
            elf: match_elf_strategy(elem[0]),
            player: match_player_strategy(elem[1]),
        };
        commands.push(tuple)
    }
    return commands;
}

fn calc_score(file_name: &str) -> i32 {
    let lines = read(file_name).expect("File read error");
    let commands = parse(&lines);
    let mut sum = 0;
    for Round { elf: e, player: p } in commands {
        let result = play(&e, &p);
        sum = sum + result;
    }
    return sum;
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
