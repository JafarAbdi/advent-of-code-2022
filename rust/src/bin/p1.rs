use std::env;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;
    let mut carried_food: Vec<u32> = input
        .split("\n\n")
        .map(|e| {
            e.split_whitespace()
                .flat_map(|calories| calories.parse::<u32>())
                .sum()
        })
        .collect();
    carried_food.sort_by(|a, b| b.cmp(a));
    if let Some(max_calories) = carried_food.first() {
        println!("Max calories: {max_calories}");
    }
    let top_3: u32 = carried_food.iter().take(3).sum();
    println!("Top 3: {top_3}");
    Ok(())
}
