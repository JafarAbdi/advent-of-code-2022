use std::{error::Error, str::FromStr};

#[derive(Debug)]
enum ShapeScore {
    Rock = 1,
    Paper,
    Scissors,
}

impl FromStr for ShapeScore {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" | "A" => Ok(ShapeScore::Rock),
            "Y" | "B" => Ok(ShapeScore::Paper),
            "Z" | "C" => Ok(ShapeScore::Scissors),
            _ => Err(()),
        }
    }
}

enum RoundScore {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for RoundScore {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

// Bi-directional map??
fn loss_shape(opponent_shape: ShapeScore) -> ShapeScore {
    match opponent_shape {
        ShapeScore::Rock => ShapeScore::Scissors,
        ShapeScore::Paper => ShapeScore::Rock,
        ShapeScore::Scissors => ShapeScore::Paper,
    }
}

fn win_shape(opponent_shape: ShapeScore) -> ShapeScore {
    match opponent_shape {
        ShapeScore::Rock => ShapeScore::Paper,
        ShapeScore::Paper => ShapeScore::Scissors,
        ShapeScore::Scissors => ShapeScore::Rock,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_strategies = std::fs::read_to_string("./p2_input.txt")?;
    // let input_strategies = "A Y
// B X
// C Z";

    let strategies: Vec<Vec<_>> = input_strategies
        .split('\n')
        .map(|strategy| {
            strategy
                .split_whitespace()
                // TODO: Is there a way without using unwrap?
                .map(|shape| ShapeScore::from_str(shape).unwrap())
                .collect()
        })
        .collect();
    let score: u32 = strategies
        .iter()
        .map(|e| match e[..] {
            [ShapeScore::Rock, ShapeScore::Rock]
            | [ShapeScore::Paper, ShapeScore::Paper]
            | [ShapeScore::Scissors, ShapeScore::Scissors] => RoundScore::Draw as u32,
            [ShapeScore::Rock, ShapeScore::Scissors]
            | [ShapeScore::Paper, ShapeScore::Rock]
            | [ShapeScore::Scissors, ShapeScore::Paper] => RoundScore::Loss as u32,
            [ShapeScore::Scissors, ShapeScore::Rock]
            | [ShapeScore::Rock, ShapeScore::Paper]
            | [ShapeScore::Paper, ShapeScore::Scissors] => RoundScore::Win as u32,
            _ => panic!("Unknown combination"),
        })
        .sum();

    // Part 1
    let pick_score: u32 = strategies.iter().map(|e| e[1] as u32).sum();
    let final_score = score + pick_score;
    println!("Final score P1: {final_score}");

    // Part 2
    let final_score: u32 = input_strategies
        .split('\n')
        .map(|strategy| {
            let s: Vec<_> = strategy.split_whitespace().collect();
            let round_outcome = RoundScore::from_str(s[1]).unwrap();
            let opponent_shape = ShapeScore::from_str(s[0]).unwrap();
            let selected_shape = match round_outcome {
                RoundScore::Loss => loss_shape(opponent_shape),
                RoundScore::Draw => opponent_shape,
                RoundScore::Win => win_shape(opponent_shape),
            };
            selected_shape as u32 + round_outcome as u32
        })
        .sum();
    println!("Final score P2: {final_score}");
    Ok(())
}
