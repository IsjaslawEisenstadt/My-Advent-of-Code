use std::env;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{anyhow, bail, Result};

#[test]
fn test_calculate_floor() -> Result<()>
{
	assert_eq!(calculate_floor(File::open("inputs/2015/day1.txt")?)?.floor, 74);
	Ok(())
}

#[test]
fn test_calculate_basement_position() -> Result<()>
{
	assert_eq!(
		calculate_floor(File::open("inputs/2015/day1.txt")?)?
			.basement_position
			.ok_or_else(|| anyhow!("Unable to find a basement position!"))?,
		1795
	);
	Ok(())
}

struct FloorInformation
{
	floor: i32,
	basement_position: Option<usize>
}

impl Display for FloorInformation
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
	{
		f.write_fmt(format_args!(
			"Floor: {}, Basement position: {}",
			self.floor,
			self.basement_position
				.map(|value| value.to_string())
				.unwrap_or_else(|| String::from("N/A"))
		))
	}
}

fn calculate_floor(instructions: File) -> Result<FloorInformation>
{
	let mut floor: i32 = 0;
	let mut basement_position: Option<usize> = None;

	for line in BufReader::new(instructions).lines()
	{
		for (i, char) in line?.chars().enumerate()
		{
			floor += match char
			{
				'(' => 1,
				')' => -1,
				_ => bail!("Invalid character found!")
			};

			if basement_position.is_none() && floor < 0
			{
				basement_position = Some(i + 1);
			}
		}
	}

	Ok(FloorInformation {
		floor,
		basement_position
	})
}

fn main() -> Result<()>
{
	let args: Vec<String> = env::args().collect();

	println!(
		"Floor information: [{}]",
		calculate_floor(File::open(
			args.get(1).ok_or_else(|| anyhow!("Must supply a filepath!"))?.as_str()
		)?)?
	);

	Ok(())
}
