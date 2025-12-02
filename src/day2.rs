#[aoc(day2, part1)]
pub fn part1(input: &str) -> u64
{
	let mut sum_of_invalids = 0;
	for line in input.as_bytes().split(|&x| x == b',')
	{
		if line.is_empty()
		{
			continue;
		}
		let mut parts = line.split(|&x| x == b'-');
		let start = parts.next().unwrap();
		let end = parts.next().unwrap();
		debug_assert!(parts.next().is_none());

		let start_len = start.len() as u32;
		let end_len = end.len() as u32;
		let start = parse_id(start);
		let end = parse_id(end);
		debug_assert!(start <= end);

		if start_len == end_len
		{
			if start_len % 2 == 0
			{
				let chunksize = 10u64.pow(start_len / 2);
				assess(start, end, chunksize, &mut sum_of_invalids);
			}
			else
			{
				// There are no invalid odd-length numbers.
			}
		}
		else if start_len + 1 == end_len
		{
			if start_len % 2 == 0
			{
				let chunksize = 10u64.pow(start_len / 2);
				let end_of_chunks = 10u64.pow(start_len) - 1;
				assess(start, end_of_chunks, chunksize, &mut sum_of_invalids);
			}
			else
			{
				let chunksize = 10u64.pow(end_len / 2);
				let start_of_chunks = 10u64.pow(end_len - 1);
				assess(start_of_chunks, end, chunksize, &mut sum_of_invalids);
			}
		}
		else
		{
			unimplemented!()
		}
	}
	sum_of_invalids
}

fn parse_id(id: &[u8]) -> u64
{
	let mut sum = 0;
	for &digit in id
	{
		debug_assert!((b'0'..=b'9').contains(&digit));
		sum *= 10;
		sum += (digit - b'0') as u64;
	}
	sum
}

fn assess(start: u64, end: u64, chunksize: u64, sum_of_invalids: &mut u64)
{
	let start_in_chunks = start / chunksize;
	let end_in_chunks = end / chunksize;
	debug_assert!(end_in_chunks < chunksize);

	for chunk in (start_in_chunks + 1)..end_in_chunks
	{
		// Guaranteed to be inside the range.
		debug_assert!(chunk < chunksize);
		let invalid_id = chunk * chunksize + chunk;
		*sum_of_invalids += invalid_id;
	}

	if end_in_chunks > start_in_chunks
	{
		let invalid_id = end_in_chunks * chunksize + end_in_chunks;
		if invalid_id <= end
		{
			*sum_of_invalids += invalid_id;
		}
	}

	let invalid_id = start_in_chunks * chunksize + start_in_chunks;
	if (start..=end).contains(&invalid_id)
	{
		*sum_of_invalids += invalid_id;
	}
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
		let given = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
		assert_eq!(part2(given), 6);
	}
}
