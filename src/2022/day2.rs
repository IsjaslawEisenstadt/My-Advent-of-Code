use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{anyhow, Context, Result};

#[test]
fn test_calculate_score_by_choosing_shape() -> Result<()>
{
	assert_eq!(
		get_score_from_strategy_guide(File::open("inputs/2022/day2.txt")?, choose_shape_by_char)?,
		11906
	);
	Ok(())
}

#[test]
fn test_calculate_score_by_choosing_outcome() -> Result<()>
{
	assert_eq!(
		get_score_from_strategy_guide(File::open("inputs/2022/day2.txt")?, choose_shape_by_outcome)?,
		11186
	);
	Ok(())
}

#[derive(PartialEq, Clone, Copy)]
enum Shape
{
	Rock = 1,
	Paper = 2,
	Scissors = 3
}

impl Shape
{
	fn get_outcome(&self, enemy_shape: Shape) -> RoundOutcome
	{
		match *self
		{
			_ if enemy_shape == *self => RoundOutcome::Draw,
			Shape::Rock if enemy_shape == Shape::Scissors => RoundOutcome::Win,
			Shape::Rock => RoundOutcome::Lose,
			Shape::Paper if enemy_shape == Shape::Rock => RoundOutcome::Win,
			Shape::Paper => RoundOutcome::Lose,
			Shape::Scissors if enemy_shape == Shape::Paper => RoundOutcome::Win,
			Shape::Scissors => RoundOutcome::Lose
		}
	}
}

enum RoundOutcome
{
	Win = 6,
	Draw = 3,
	Lose = 0
}

fn choose_shape_by_char(_: Shape, player_shape_char: &char) -> Result<Shape>
{
	match player_shape_char
	{
		'X' => Ok(Shape::Rock),
		'Y' => Ok(Shape::Paper),
		'Z' => Ok(Shape::Scissors),
		_ => Err(anyhow!("Invalid player shape: {}", player_shape_char))
	}
}

fn choose_shape_by_outcome(enemy_shape: Shape, player_outcome_char: &char) -> Result<Shape>
{
	match player_outcome_char
	{
		'X' => Ok(match enemy_shape
		{
			Shape::Rock => Shape::Scissors,
			Shape::Paper => Shape::Rock,
			Shape::Scissors => Shape::Paper
		}),
		'Y' => Ok(enemy_shape),
		'Z' => Ok(match enemy_shape
		{
			Shape::Rock => Shape::Paper,
			Shape::Paper => Shape::Scissors,
			Shape::Scissors => Shape::Rock
		}),
		_ => Err(anyhow!("Invalid player outcome: {}", player_outcome_char))
	}
}

fn get_score_from_strategy_guide<S>(strategy_guide: File, player_strategy: S) -> Result<u32>
where S: Fn(Shape, &char) -> Result<Shape>
{
	let mut score: u32 = 0;

	for (i, line) in BufReader::new(strategy_guide).lines().enumerate()
	{
		let line_chars: Vec<char> = line
			.with_context(|| format!("Failed to read line {}!", i))?
			.chars()
			.collect::<Vec<char>>();

		let enemy_shape: Shape = match line_chars
			.first()
			.ok_or_else(|| anyhow!("Line {} is missing an enemy shape!", i))?
		{
			'A' => Ok(Shape::Rock),
			'B' => Ok(Shape::Paper),
			'C' => Ok(Shape::Scissors),
			char => Err(anyhow!("Invalid enemy shape: {}", char))
		}?;

		let player_shape: Shape = player_strategy(
			enemy_shape,
			line_chars
				.get(2)
				.ok_or_else(|| anyhow!("Line {} is missing a player shape!", i))?
		)?;

		let round_outcome: RoundOutcome = player_shape.get_outcome(enemy_shape);

		score += player_shape as u32 + round_outcome as u32;
	}

	Ok(score)
}

fn main() -> Result<()>
{
	let args: Vec<String> = env::args().collect();

	println!(
		"The score from this strategy guide is: {}",
		get_score_from_strategy_guide(
			File::open(args.get(1).ok_or_else(|| anyhow!("Must supply a filepath!"))?.as_str())?,
			if args.get(2).unwrap_or(&String::from("c")).eq_ignore_ascii_case("o")
			{
				choose_shape_by_outcome
			}
			else
			{
				choose_shape_by_char
			}
		)?
	);

	Ok(())
}
