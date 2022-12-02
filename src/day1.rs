use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{anyhow, bail, Result};

#[test]
fn test_find_most_calories_for_one_elf() -> Result<()> {
	assert_eq!(find_highest_calories(File::open("inputs/day1.txt")?, 1)?, 66616);
	Ok(())
}

#[test]
fn test_find_most_calories_for_three_elfs() -> Result<()> {
	assert_eq!(find_highest_calories(File::open("inputs/day1.txt")?, 3)?, 199172);
	Ok(())
}

fn find_highest_calories(file: File, num_elfs: usize) -> Result<u32> {
	if num_elfs == 0 {
		bail!("Number of elf's must be greater than 0!");
	}

	let reader = BufReader::new(file);
	let mut current_elf_calories: u32 = 0;
	let mut highest_calories: Vec<u32> = std::iter::repeat(0).take(num_elfs).collect();

	for line in reader.lines() {
		if let Ok(parse_result) = line?.parse::<u32>() {
			current_elf_calories += parse_result;
		} else {
			for calories in highest_calories.iter().rev() {
				if current_elf_calories > *calories {
					highest_calories.push(current_elf_calories);
					highest_calories.remove(0);
					break;
				}
			}
			current_elf_calories = 0;
		}
	}

	Ok(highest_calories.iter().sum())
}

fn main() -> Result<()> {
	let args = env::args().collect::<Vec<String>>();

	println!(
		"The highest calories are {}",
		find_highest_calories(
			File::open(args.get(1).ok_or_else(|| anyhow!("Must supply a filepath!"))?.as_str())?,
			args.get(2).unwrap_or(&String::from("1")).parse::<usize>().unwrap_or(1)
		)?
	);

	Ok(())
}
