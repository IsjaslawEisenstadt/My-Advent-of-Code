use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{anyhow, Result};

#[test]
fn test_contained_assignment_count() -> Result<()>
{
	let (fully_contained, partially_contained) = get_contained_assignment_count(File::open("inputs/2022/day4.txt")?)?;
	assert_eq!(fully_contained, 644);
	assert_eq!(partially_contained, 926);
	Ok(())
}

fn get_contained_assignment_count(assignments: File) -> Result<(u32, u32)>
{
	let mut fully_contained_count: u32 = 0;
	let mut partially_contained_count: u32 = 0;

	for line_res in BufReader::new(assignments).lines()
	{
		let line = line_res?;
		let mut pairs = line.split(',');
		let assignment_err = || anyhow!("Missing assignment1!");
		let mut assignment1 = pairs.next().ok_or_else(assignment_err)?.split('-');
		let mut assignment2 = pairs.next().ok_or_else(assignment_err)?.split('-');

		let assignment1_lower_end = assignment1.next().ok_or_else(|| anyhow!("Missing assignment1 lower end!"))?.parse::<u32>()?;
		let assignment1_upper_end = assignment1.next().ok_or_else(|| anyhow!("Missing assignment1 upper end!"))?.parse::<u32>()?;
		let assignment2_lower_end = assignment2.next().ok_or_else(|| anyhow!("Missing assignment2 lower end!"))?.parse::<u32>()?;
		let assignment2_upper_end = assignment2.next().ok_or_else(|| anyhow!("Missing assignment2 upper end!"))?.parse::<u32>()?;

		if (assignment1_lower_end <= assignment2_lower_end && assignment1_upper_end >= assignment2_upper_end)
			|| (assignment2_lower_end <= assignment1_lower_end && assignment2_upper_end >= assignment1_upper_end)
		{
			fully_contained_count += 1;
		}

		if (assignment1_lower_end >= assignment2_lower_end && assignment1_lower_end <= assignment2_upper_end)
			|| (assignment2_lower_end >= assignment1_lower_end && assignment2_lower_end <= assignment1_upper_end)
		{
			partially_contained_count += 1;
		}
	}

	Ok((fully_contained_count, partially_contained_count))
}

fn main() -> Result<()>
{
	let (fully_contained, partially_contained) = get_contained_assignment_count(File::open(
		env::args().collect::<Vec<String>>().get(1).ok_or_else(|| anyhow!("Must supply a filepath!"))?.as_str()
	)?)?;

	println!(
		"The fully contained count is: {} and the partially contained count is: {}",
		fully_contained, partially_contained
	);

	Ok(())
}
