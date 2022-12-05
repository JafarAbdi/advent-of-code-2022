use std::{error::Error, fs, env};

use itertools::Itertools;

#[derive(Debug)]
struct Range {
    lower: u32,
    upper: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.lower <= other.lower && other.upper <= self.upper
    }
    fn overlaps(&self, other: &Range) -> bool {
        self.lower <= other.upper && other.lower <= self.upper
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;
    let parsed: Vec<_> = input
        .lines()
        .map(|e| {
            if let Some((range1, range2)) = e.split(',').collect_tuple() {
                let (range1_min, range1_max) = range1.split('-').collect_tuple().unwrap();
                let (range2_min, range2_max) = range2.split('-').collect_tuple().unwrap();
                [
                    Range {
                        lower: range1_min.parse().unwrap(),
                        upper: range1_max.parse().unwrap(),
                    },
                    Range {
                        lower: range2_min.parse().unwrap(),
                        upper: range2_max.parse().unwrap(),
                    },
                ]
            } else {
                panic!("Failed to parse an input!")
            }
        })
        .collect();

    let s1: u32 = parsed
        .iter()
        .map(|e| {
            let [range1, range2] = e;
            (range1.contains(range2) || range2.contains(range1)) as u32
        })
        .sum();
    println!("P1: {s1}");
    let s2: u32 = parsed
        .iter()
        .map(|e| {
            let [range1, range2] = e;
            range1.overlaps(range2) as u32
        })
        .sum();
    println!("P2: {s2}");
    Ok(())
}
