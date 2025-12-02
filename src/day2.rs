#[aoc(day2, part1)]
pub fn part1(input: &str) -> u64
{
	parse_input(input)
		.map(|(start, end)| assess_part1(start, end))
		.sum()
}

#[derive(Clone, Copy, Debug)]
struct Id
{
	value: u64,
	len: u32,
}

fn parse_id(id: &[u8]) -> Id
{
	let len = id.len() as u32;
	let mut sum = 0;
	for &digit in id
	{
		debug_assert!((b'0'..=b'9').contains(&digit));
		sum *= 10;
		sum += (digit - b'0') as u64;
	}
	let value = sum;
	Id { value, len }
}

fn parse_input(input: &str) -> impl Iterator<Item = (Id, Id)>
{
	input
		.as_bytes()
		.split(|&x| x == b',')
		.filter(|x| !x.is_empty())
		.map(|line| {
			let mut parts = line.split(|&x| x == b'-');
			let start = parts.next().unwrap();
			let end = parts.next().unwrap();
			debug_assert!(parts.next().is_none());
			let start = parse_id(start);
			let end = parse_id(end);
			(start, end)
		})
}

fn assess_part1(start: Id, end: Id) -> u64
{
	debug_assert!(start.value <= end.value);

	if start.len == end.len
	{
		if start.len % 2 == 0
		{
			let chunksize = 10u64.pow(start.len / 2);
			assess2(start.value, end.value, chunksize)
		}
		else
		{
			// There are no invalid odd-length numbers.
			0
		}
	}
	else if start.len + 1 == end.len
	{
		if start.len % 2 == 0
		{
			let chunksize = 10u64.pow(start.len / 2);
			let end_of_chunks = 10u64.pow(start.len) - 1;
			assess2(start.value, end_of_chunks, chunksize)
		}
		else
		{
			let chunksize = 10u64.pow(end.len / 2);
			let start_of_chunks = 10u64.pow(end.len - 1);
			assess2(start_of_chunks, end.value, chunksize)
		}
	}
	else
	{
		unimplemented!()
	}
}

fn assess2(start: u64, end: u64, chunksize: u64) -> u64
{
	let mut sum_of_invalids = 0;
	let start_in_chunks = start / chunksize;
	let end_in_chunks = end / chunksize;
	debug_assert!(end_in_chunks < chunksize);

	for chunk in (start_in_chunks + 1)..end_in_chunks
	{
		// Guaranteed to be inside the range.
		debug_assert!(chunk < chunksize);
		let invalid_id = chunk * chunksize + chunk;
		sum_of_invalids += invalid_id;
	}

	if end_in_chunks > start_in_chunks
	{
		let invalid_id = end_in_chunks * chunksize + end_in_chunks;
		if invalid_id <= end
		{
			sum_of_invalids += invalid_id;
		}
	}

	let invalid_id = start_in_chunks * chunksize + start_in_chunks;
	if (start..=end).contains(&invalid_id)
	{
		sum_of_invalids += invalid_id;
	}
	sum_of_invalids
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u64
{
	parse_input(input)
		.map(|(start, end)| assess_part2(start, end))
		.sum()
}

fn assess_part2(start: Id, end: Id) -> u64
{
	// if len is 8, don't count 2's because they are already counted as 4's
	// only count 1's if len is prime
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
