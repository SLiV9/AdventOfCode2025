#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64
{
	input
		.lines()
		.filter(|x| !x.is_empty())
		.map(find_largest_safe_joltage_in_bank)
		.sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64
{
	input
		.lines()
		.filter(|x| !x.is_empty())
		.map(find_largest_overcharged_joltage_in_bank)
		.sum()
}

fn find_largest_safe_joltage_in_bank(bank: &str) -> u64
{
	let digits = bank.as_bytes();
	assert!(digits.len() >= 2);
	debug_assert!(digits.iter().all(|x| x.is_ascii_digit()));
	let mut start = 0;
	let mut first_digit = digits[0];
	for i in 1..(digits.len() - 1)
	{
		if digits[i] > first_digit
		{
			first_digit = digits[i];
			start = i;
		}
	}
	debug_assert!(start + 1 < digits.len());
	let second_digit = digits[(start + 1)..].iter().copied().max().unwrap();
	u64::from(first_digit - b'0') * 10 + u64::from(second_digit - b'0')
}

fn find_largest_overcharged_joltage_in_bank(bank: &str) -> u64
{
	let digits = bank.as_bytes();
	let mut leeway = digits.len() - 12;
	let mut joltage = 0;
	let mut start = 0;
	for _ in 0..12
	{
		let mut best_digit = digits[start];
		let mut digits_skipped = 0;
		for skip in 1..=leeway
		{
			let i = start + skip;
			if digits[i] > best_digit
			{
				best_digit = digits[i];
				digits_skipped = skip;
			}
		}
		start += 1 + digits_skipped;
		leeway -= digits_skipped;
		joltage *= 10;
		joltage += u64::from(best_digit - b'0');
	}
	joltage
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::*;

	#[test]
	fn test_part1_given()
	{
		let given = "987654321111111
811111111111119
234234234234278
818181911112111";
		assert_eq!(part1(given), 357);
	}

	#[test]
	fn test_part2_given()
	{
		let given = "987654321111111
811111111111119
234234234234278
818181911112111";
		assert_eq!(part2(given), 3121910778619);
	}

	#[test]
	fn test_part1_testcases()
	{
		assert_eq!(part1("12"), 12);
		assert_eq!(part1("123"), 23);
		assert_eq!(part1("4341"), 44);
	}

	#[test]
	fn test_part2_testcases()
	{
		assert_eq!(part2("234234234234278"), 434234234278);
		assert_eq!(part2("281111111111119"), 811111111119);
	}
}
