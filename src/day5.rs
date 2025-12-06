#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
	let mut toggles = Vec::with_capacity(512);
	toggles.push(u64::MAX);
	let mut lines = input.lines();
	loop {
		let line = lines.next().unwrap();
		if line.is_empty() {
			break;
		}
		let (a, b) = parse_range(line);
		add_inclusive_range_to_toggle_set(&mut toggles, a, b);
	}
	lines
		.map(|x| x.parse().unwrap())
		.filter(|x: &u64| is_in_toggle_set(&toggles, *x))
		.count()
}

fn parse_range(line: &str) -> (u64, u64) {
	let mut parts = line.split('-');
	let left = parts.next().unwrap();
	let right = parts.next().unwrap();
	debug_assert!(parts.next().is_none());
	(left.parse().unwrap(), right.parse().unwrap())
}

fn add_inclusive_range_to_toggle_set(toggles: &mut Vec<u64>, a: u64, b_inclusive: u64) {
	debug_assert!(a <= b_inclusive);
	debug_assert!(b_inclusive < u64::MAX);
	let b = b_inclusive.wrapping_add(1);
	let i_a = toggles.binary_search(&a);
	let i_b = toggles.binary_search(&b);

	let include_a;
	let include_b;
	let splice_start;
	let splice_end;

	match i_a {
		Err(i_a) if i_a % 2 == 0 => {
			include_a = true;
			splice_start = i_a;
		}
		Ok(i_a) if i_a % 2 == 0 => {
			include_a = true;
			splice_start = i_a;
		}
		Err(i_a) => {
			include_a = false;
			splice_start = i_a + 1;
		}
		Ok(i_a) => {
			include_a = false;
			splice_start = i_a;
		}
	}

	match i_b {
		Err(i_b) if i_b % 2 == 0 => {
			include_b = true;
			splice_end = i_b;
		}
		Ok(i_b) if i_b % 2 == 0 => {
			include_b = false;
			splice_end = i_b + 1;
		}
		Err(i_b) => {
			include_b = false;
			splice_end = i_b;
		}
		Ok(i_b) => {
			include_b = false;
			splice_end = i_b;
		}
	}

	let ab = [a, b];
	let include_start = if include_a { 0 } else { 1 };
	let include_end = if include_b { 2 } else { 1 };
	let included = &ab[include_start..include_end];

	if splice_start > splice_end {
		debug_assert!(included.is_empty());
		return;
	}

	toggles.splice(splice_start..splice_end, included.into_iter().copied());
}

fn is_in_toggle_set(toggles: &[u64], x: u64) -> bool {
	debug_assert!(x < u64::MAX);
	// If it is > toggles[0] and < toggles[1], it is in the set.
	// If it is == toggles[0], it is in the set.
	// If it is == toggles[1], it is not in the set.
	match toggles.binary_search(&x) {
		Ok(i) => i % 2 == 0,
		Err(i) => i % 2 == 1,
	}
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
	input.len()
}

#[cfg(test)]
mod tests {
	use pretty_assertions::assert_eq;

	use super::*;

	fn init_logger() {
		env_logger::Builder::new()
			.filter_level(log::LevelFilter::Debug)
			.init();
	}

	#[test]
	fn test_part1_given() {
		init_logger();

		let given = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
		assert_eq!(part1(given), 3);
	}

	#[test]
	fn test_part2_given() {
		init_logger();

		let given = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
		assert_eq!(part2(given), 43);
	}

	#[test]
	fn test_is_in_toggle_set() {
		let check = |toggles: &[u64], x: u64, expected: bool| {
			let mut toggles = toggles.to_vec();
			toggles.push(u64::MAX);
			assert_eq!(is_in_toggle_set(&toggles, x), expected);
		};

		check(&[10, 20], 0, false);
		check(&[10, 20], 9, false);
		check(&[10, 20], 10, true);
		check(&[10, 20], 15, true);
		check(&[10, 20], 19, true);
		check(&[10, 20], 20, false);
		check(&[10, 20], 12378123, false);
		check(&[10, 20, 21, 22], 20, false);
		check(&[10, 20, 21, 22], 21, true);
		check(&[10, 20, 21, 22], 22, false);
		check(&[10, 20, 21, 22], 23, false);
	}

	#[test]
	fn test_add_inclusive_range_to_toggle_set() {
		let check = |before: &[u64], a: u64, b: u64, after: &[u64]| {
			let mut toggles = before.to_vec();
			toggles.push(u64::MAX);
			add_inclusive_range_to_toggle_set(&mut toggles, a, b);
			assert_eq!(&toggles[..toggles.len() - 1], after);
		};

		check(&[], 10, 19, &[10, 20]);
		check(&[10, 20], 30, 39, &[10, 20, 30, 40]);
		check(&[10, 20], 11, 11, &[10, 20]);
		check(&[10, 20], 30, 30, &[10, 20, 30, 31]);
		check(&[10, 20], 20, 20, &[10, 21]);
		check(&[10, 20], 10, 10, &[10, 20]);
		check(&[10, 20], 9, 9, &[9, 20]);
		check(&[10, 20], 5, 25, &[5, 26]);
	}
}
