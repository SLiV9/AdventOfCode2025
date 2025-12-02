#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32
{
	let mut sum_of_invalids = 0;
	for line in input.as_bytes().split(|&x| x == b',')
	{
		if line.is_empty()
		{
			continue;
		}
		let parts = line.split(|&x| x == b'-');
		for id in parts
		{
			if is_invalid(id)
			{
                let id: u32 = parse_id(id);
				sum_of_invalids += id;
			}
		}
	}
	sum_of_invalids
}

fn is_invalid(id: &[u8]) -> bool
{
	if id.len() % 2 != 0
	{
		return false;
	}
	let (left, right) = id.split_at(id.len() / 2);
	left == right
}

fn parse_id(id: &[u8]) -> u32 {
    let mut sum = 0;
    for &digit in id {
        debug_assert!((b'0'..=b'9').contains(&digit));
        sum *= 10;
        sum += (digit - b'0') as u32;
    }
    sum
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32
{
	input.len() as i32
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::*;

	#[test]
	fn test_day1_part1_given()
	{
		let given = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
		assert_eq!(part1(given), 1227775554);
	}

	#[test]
	fn test_day1_part2_given()
	{
		let given = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
		assert_eq!(part2(given), 6);
	}
}
