#[derive(Clone, Copy)]
enum Choice {
    Rock, Paper, Scissors
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose, Draw, Win
}

fn calculate(yours: Choice, theirs: Choice) -> i32 {
    let choice_value = match yours {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3
    };
    let outcome_value = match (yours, theirs) {
        (Choice::Rock, Choice::Rock) => 3,
        (Choice::Rock, Choice::Paper) => 0,
        (Choice::Rock, Choice::Scissors) => 6,
        (Choice::Paper, Choice::Rock) => 6,
        (Choice::Paper, Choice::Paper) => 3,
        (Choice::Paper, Choice::Scissors) => 0,
        (Choice::Scissors, Choice::Rock) => 0,
        (Choice::Scissors, Choice::Paper) => 6,
        (Choice::Scissors, Choice::Scissors) => 3,
    };
    choice_value + outcome_value
}

fn calculate2(desired: Outcome, theirs: Choice) -> i32 {
    let (outcome_value, your_choice) = match (desired, theirs) {
        (Outcome::Lose, Choice::Rock) => (0, Choice::Scissors),
        (Outcome::Lose, Choice::Paper) => (0, Choice::Rock),
        (Outcome::Lose, Choice::Scissors) => (0, Choice::Paper),
        (Outcome::Draw, _) => (3, theirs),
        (Outcome::Win, Choice::Rock) => (6, Choice::Paper),
        (Outcome::Win, Choice::Paper) => (6, Choice::Scissors),
        (Outcome::Win, Choice::Scissors) => (6, Choice::Rock),
    };
    let choice_value = match your_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3
    };
    choice_value + outcome_value
}

fn main() {
    let input = include_str!("day2_input.txt").trim();
    let mut running_total_part1 = 0;
    let mut running_total_part2 = 0;
    for line in input.lines() {
        let mut splits = line.split_ascii_whitespace();

        let opponent = match splits.next().unwrap() {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => panic!(":)")
        };

        let your_value = splits.next().unwrap();
        let you_attack = match your_value {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => panic!(":)")
        };
        let you_outcome = match your_value {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!(":)")
        };

        running_total_part1 += calculate(you_attack, opponent);
        running_total_part2 += calculate2(you_outcome, opponent);
    }
    println!("Part one: {}", running_total_part1);
    println!("Part two: {}", running_total_part2);
}
