use itertools::Itertools;
use std::error::Error;
use std::{env, fs};

fn char_to_priority(c: char) -> u32 {
    if c.is_uppercase() {
        return (c as u32) - 65 + 27;
    }
    (c as u32) - 96
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;
    let priorities: u32 = input
        .split('\n')
        .map(|e| {
            let (first_compartment, second_compartment) = e.split_at(e.len() / 2);
            for c in first_compartment.chars() {
                if second_compartment.contains(c) {
                    return char_to_priority(c);
                }
            }
            panic!("Unexpected input '{e}'");
        })
        .sum();
    println!("P1: {priorities}");

    let mut priorities = 0;
    for (elf_1, elf_2, elf_3) in input.lines().tuples() {
        priorities += elf_1
            .chars()
            .filter(|c| elf_2.contains(*c) && elf_3.contains(*c))
            .map(char_to_priority)
            .unique()
            .sum::<u32>();
    }
    println!("P2: {priorities}");
    Ok(())
}
