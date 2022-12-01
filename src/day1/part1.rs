use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::{env, io};

#[test]
fn test_find_most_calories() -> Result<(), io::Error> {
	assert_eq!(find_most_calories(File::open("inputs/d1p1.txt")?)?, 66616);
	Ok(())
}

fn find_most_calories(file: File) -> Result<i32, io::Error> {
	let reader = BufReader::new(file);

	let mut current_elf_calories = 0;
	let mut highest_elf_calories = 0;

	for line in reader.lines() {
		if let Ok(parse_result) = line?.parse::<i32>() {
			current_elf_calories += parse_result;
		} else {
			if current_elf_calories > highest_elf_calories {
				highest_elf_calories = current_elf_calories
			}
			current_elf_calories = 0;
		}
	}

	Ok(highest_elf_calories)
}

fn main() -> Result<(), io::Error> {
	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		println!(
			"The highest calories are {}",
			find_most_calories(File::open(args[1].as_str())?)?
		);
	} else {
		println!("Must supply a filepath!");
	}
	Ok(())
}
