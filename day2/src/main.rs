//      them     me
// Rock A        X
// Paper B       Y
// Scissors C    Z

// Round score:
// Shape selected:
//   - Rock 1
//   - Paper 2
//   - Scissors 3
// Outcome of round
//   - win 6
//   - draw 3
//   - loss 0

use std::fs;

#[derive(Debug, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Round {
    i_played: Hand,
    they_played: Hand,
}

impl Round {
    fn new(i_played: Hand, they_played: Hand) -> Self {
        Self {
            i_played,
            they_played,
        }
    }

    fn get_my_score(&self) -> i32 {
        let mut score: i32 = 0;

        // My 'hand score'
        match self.i_played {
            Hand::Rock => score += 1,
            Hand::Paper => score += 2,
            Hand::Scissors => score += 3,
        }

        match (&self.i_played, &self.they_played) {
            // Winning scenarios
            (Hand::Rock, Hand::Scissors) => score += 6,
            (Hand::Paper, Hand::Rock) => score += 6,
            (Hand::Scissors, Hand::Paper) => score += 6,

            // Losing scenarios
            (Hand::Rock, Hand::Paper) => {}
            (Hand::Paper, Hand::Scissors) => {}
            (Hand::Scissors, Hand::Rock) => {}

            // Draw
            (_, _) => score += 3,
        }

        score
    }
}

fn get_hand_from_char(input_char: &char) -> Hand {
    match input_char {
        'X' => Hand::Rock,
        'Y' => Hand::Paper,
        'Z' => Hand::Scissors,
        'A' => Hand::Rock,
        'B' => Hand::Paper,
        'C' => Hand::Scissors,
        _ => panic!(),
    }
}

fn main() {
    let file_name = "src/input.txt";
    let file_data: String = fs::read_to_string(file_name).unwrap();
    let lines: Vec<&str> = file_data.lines().collect();

    solve1(&lines);
    solve2(&lines);
}

fn solve1(lines: &Vec<&str>) {
    let mut my_score = 0;
    for line in lines {
        let round_hands: Vec<char> = line.chars().collect();

        let they_played = get_hand_from_char(round_hands.get(0).unwrap());
        let i_played = get_hand_from_char(round_hands.get(2).unwrap());

        let round = Round::new(i_played, they_played);
        my_score += round.get_my_score();
    }

    print!("My score is: {}", my_score)
}

fn solve2(lines: &Vec<&str>) {
    let mut my_score = 0;
    for line in lines {
        let round_hands: Vec<char> = line.chars().collect();

        let they_played = get_hand_from_char(round_hands.get(0).unwrap());
        let outcome_char = round_hands.get(2).unwrap();

        // X , i lose
        // y , draw
        // z , i win
        let i_played: Hand = match (&they_played, outcome_char) {
            (Hand::Rock, 'X') => Hand::Scissors,
            (Hand::Paper, 'X') => Hand::Rock,
            (Hand::Scissors, 'X') => Hand::Paper,

            (Hand::Rock, 'Z') => Hand::Paper,
            (Hand::Paper, 'Z') => Hand::Scissors,
            (Hand::Scissors, 'Z') => Hand::Rock,

            (their_hand, 'Y') => their_hand.clone(),
            (_, _) => panic!(),
        };

        let round = Round::new(i_played, they_played);
        my_score += round.get_my_score();
    }

    print!("My score is: {}", my_score)
}
