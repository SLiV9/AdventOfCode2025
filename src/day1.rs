use parse_display::{Display, FromStr};

#[derive(Debug, PartialEq)]
struct Instruction
{
	direction: Direction,
	amount: i32,
}

#[derive(Clone, Copy, Display, FromStr, Debug, PartialEq)]
enum Direction
{
	#[display("L")]
	Left,
	#[display("R")]
	Right,
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32
{
	let mut dial: u32 = 50;
	let mut times_zero = 0;
	for line in input.lines()
	{
		if line.is_empty()
		{
			continue;
		}
		let (letter, number) = line.split_at(1);
		let direction = letter.parse().unwrap();
		let amount: u32 = number.parse().unwrap();
		let amount = amount % 100;
		dial += match direction
		{
			Direction::Left => 100 - amount,
			Direction::Right => amount,
		};
		dial = dial % 100;
		if dial == 0
		{
			times_zero += 1;
		}
	}
	times_zero
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32
{
	todo!()
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::*;

	#[test]
	fn test_day1_part1_given()
	{
		let given = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
		assert_eq!(part1(given), 3);
	}

	// #[test]
	// fn test_day1_part2_given()
	// {
	// 	let given = "3   4
	//         4   3
	//         2   5
	//         1   3
	//         3   9
	//         3   3";
	// 	assert_eq!(part2(given), 31);
	// }
}
