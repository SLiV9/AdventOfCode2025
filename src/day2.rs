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
			assess2(start.len, start.value, end.value)
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
			let mid_value = 10u64.pow(start.len) - 1;
			assess2(start.len, start.value, mid_value)
		}
		else
		{
			let mid_value = 10u64.pow(end.len - 1);
			assess2(end.len, mid_value, end.value)
		}
	}
	else
	{
		unimplemented!()
	}
}

fn assess2(len: u32, start: u64, end: u64) -> u64
{
	match len
	{
		0 => 0,
		2 => eval::<2, 1>(start, end),
		4 => eval::<2, 2>(start, end),
		6 => eval::<2, 3>(start, end),
		8 => eval::<2, 4>(start, end),
		10 => eval::<2, 5>(start, end),
		12.. => unimplemented!(),
		_ => unreachable!(),
	}
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
		assess_part2_equal_len(start, end)
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
		assess_part2_equal_len(start, mid0) + assess_part2_equal_len(mid1, end)
	}
}

fn assess_part2_equal_len(start: Id, end: Id) -> u64
{
	debug_assert_eq!(start.len, end.len);
	let len = start.len;
	let a = start.value;
	let b = end.value;
	match len
	{
		0 => 0,
		1 => 0,
		2 => eval::<2, 1>(a, b),
		3 => eval::<3, 1>(a, b),
		4 => eval::<2, 2>(a, b),
		5 => eval::<5, 1>(a, b),
		6 => eval::<2, 3>(a, b) + eval::<3, 2>(a, b) - eval::<6, 1>(a, b),
		7 => eval::<7, 1>(a, b),
		8 => eval::<2, 4>(a, b),
		9 => eval::<3, 3>(a, b),
		10 => eval::<2, 5>(a, b) + eval::<5, 2>(a, b) - eval::<10, 1>(a, b),
		11.. => unimplemented!(),
	}
}

fn eval<const DIVISOR: u32, const QUOTIENT: u32>(start: u64, end: u64) -> u64
{
	let build_invalid_id = |chunk| {
		let mut invalid_id = chunk;
		for _ in 1..DIVISOR
		{
			invalid_id *= 10u64.pow(QUOTIENT);
			invalid_id += chunk;
		}
		invalid_id
	};

	let chunksize = 10u64.pow((DIVISOR - 1) * QUOTIENT);
	let start_in_chunks = start / chunksize;
	let end_in_chunks = end / chunksize;

	let mut sum_of_invalids = 0;
	for chunk in (start_in_chunks + 1)..end_in_chunks
	{
		let invalid_id = build_invalid_id(chunk);
		sum_of_invalids += invalid_id;
	}

	if end_in_chunks > start_in_chunks
	{
		let invalid_id = build_invalid_id(end_in_chunks);
		if invalid_id <= end
		{
			sum_of_invalids += invalid_id;
		}
	}

	let invalid_id = build_invalid_id(start_in_chunks);
	if (start..=end).contains(&invalid_id)
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
		for _length in 2..=10
		{
			let input = format!("{id}-{id}");
			assert_eq!(part2(&input), id);

			id *= 10;
			id += 2;
		}
	}
}
