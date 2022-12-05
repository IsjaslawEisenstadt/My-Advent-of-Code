use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{anyhow, Result};

#[test]
fn test_get_priority_sum_for_compartments() -> Result<()>
{
	assert_eq!(get_priority_sum_for_compartments(&File::open("inputs/2022/day3.txt")?)?, 7763);
	Ok(())
}

#[test]
fn test_get_priority_sum_for_groups() -> Result<()>
{
	assert_eq!(get_priority_sum_for_groups(&File::open("inputs/2022/day3.txt")?)?, 2569);
	Ok(())
}

fn get_priority_sum_for_compartments(rucksacks: &File) -> Result<u32>
{
	let mut priority_sum: u32 = 0;

	for line_res in BufReader::new(rucksacks).lines()
	{
		let line = line_res?;
		let (compartment1, compartment2) = line.split_at(line.len() / 2);
		let compartment1_set: HashSet<char> = HashSet::from_iter(compartment1.chars());

		for item in compartment2.chars()
		{
			if compartment1_set.contains(&item)
			{
				priority_sum += get_priority_for_item(item)?;
				break;
			}
		}
	}

	Ok(priority_sum)
}

fn get_priority_sum_for_groups(rucksacks: &File) -> Result<u32>
{
	let mut priority_sum: u32 = 0;
	let mut lines = BufReader::new(rucksacks).lines();

	while let Some(rucksack1_line) = lines.next()
	{
		let rucksack_err = || anyhow!("Found a Rucksack without a counterpart!");
		let rucksack2_line = lines.next().ok_or_else(rucksack_err)??;
		let rucksack3_line = lines.next().ok_or_else(rucksack_err)??;

		let rucksack1_set: HashSet<char> = HashSet::from_iter(rucksack1_line?.chars());
		let mut badge: Option<char> = None;

		'outer: for rucksack2_char in rucksack2_line.chars()
		{
			if rucksack1_set.contains(&rucksack2_char)
			{
				for rucksack3_char in rucksack3_line.chars()
				{
					if rucksack3_char == rucksack2_char
					{
						badge = Some(rucksack3_char);
						break 'outer;
					}
				}
			}
		}

		priority_sum += get_priority_for_item(badge.ok_or_else(|| anyhow!("Unable to find the badge!"))?)?;
	}

	Ok(priority_sum)
}

fn get_priority_for_item(item: char) -> Result<u32>
{
	match item
	{
		_ if item.is_ascii_alphabetic() && item.is_lowercase() => Ok(item as u32 - 96),
		_ if item.is_ascii_alphabetic() && item.is_uppercase() => Ok(item as u32 - 38),
		_ => Err(anyhow!("Invalid character found: {}", item))
	}
}

fn main() -> Result<()>
{
	let args: Vec<String> = env::args().collect();

	let mut rucksacks = File::open(args.get(1).ok_or_else(|| anyhow!("Must supply a filepath!"))?.as_str())?;

	println!("The compartment sum is: {}", get_priority_sum_for_compartments(&rucksacks)?);
	rucksacks.rewind()?;
	println!("The group sum is: {}", get_priority_sum_for_groups(&rucksacks)?);

	Ok(())
}
