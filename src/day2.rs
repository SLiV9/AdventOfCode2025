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
	if start.len == end.len
	{
		assess_part2_(start, end)
	}
	else
	{
		debug_assert_eq!(start.len + 1, end.len);
		let mid = 10u64.pow(start.len);
		let mid0 = Id {
			value: mid - 1,
			len: start.len,
		};
		let mid1 = Id {
			value: mid,
			len: end.len,
		};
		assess_part2_(start, mid0) + assess_part2_(mid1, end)
	}
}

const PRIMES: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
const DOUBLETS: [u32; 4] = [6, 10, 14, 15];

fn assess_part2_(start: Id, end: Id) -> u64
{
	let sum_of_invalids: u64 = PRIMES
		.iter()
		.copied()
		.map(|prime| assess_divisor(start, end, prime))
		.sum();
	let sum_of_double_counted: u64 = DOUBLETS
		.iter()
		.copied()
		.map(|divisor| assess_divisor(start, end, divisor))
		.sum();
	sum_of_invalids - sum_of_double_counted
}

fn assess_divisor(start: Id, end: Id, divisor: u32) -> u64
{
	if start.len % divisor != 0
	{
		return 0;
	}
	let mut sum_of_invalids = 0;

	let num_digits_per_chunk = start.len / divisor;

	let build_invalid_id = |chunk| {
		let mut invalid_id = chunk;
		for _ in 1..divisor
		{
			invalid_id *= 10u64.pow(num_digits_per_chunk);
			invalid_id += chunk;
		}
		invalid_id
	};

	let chunksize = 10u64.pow(start.len - num_digits_per_chunk);
	let start_in_chunks = start.value / chunksize;
	let end_in_chunks = end.value / chunksize;

	for chunk in (start_in_chunks + 1)..end_in_chunks
	{
		let invalid_id = build_invalid_id(chunk);
		sum_of_invalids += invalid_id;
	}

	if end_in_chunks > start_in_chunks
	{
		let invalid_id = build_invalid_id(end_in_chunks);
		if invalid_id <= end.value
		{
			sum_of_invalids += invalid_id;
		}
	}

	let invalid_id = build_invalid_id(start_in_chunks);
	if (start.value..=end.value).contains(&invalid_id)
	{
		sum_of_invalids += invalid_id;
	}

	sum_of_invalids
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::*;

	#[test]
	fn test_part1_given()
	{
		let given = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
		assert_eq!(part1(given), 1227775554);
	}

	#[test]
	fn test_part2_given()
	{
		let given = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
		assert_eq!(part2(given), 4174379265);
	}

	#[test]
	fn test_part2_duplicates()
	{
		let mut id = 22;
		for _ in 0..15
		{
			let input = format!("{id}-{id}");
			assert_eq!(part2(&input), id);

			id *= 10;
			id += 2;
		}
	}
}
