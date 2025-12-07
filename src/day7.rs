#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
	let mut bitmask = [false; 1024];
	let mut number_of_splits = 0;
	let mut lines = input.lines();
	let first_line = lines.next().unwrap();
	let first_line = first_line.as_bytes();
	// Adding one to simplify logic, and rounding up to the nearest multiple of 8.
	let n = (first_line.len() / 8 + 1) * 8;
	let bitmask = &mut bitmask[..n];
	let start_pos = first_line.iter().copied().position(|x| x == b'S').unwrap();
	bitmask[start_pos] = true;
	for line in lines {
		let line = line.as_bytes();
		let k = n.min(line.len());
		let mut delayed_store = false;
		for i in 1..k {
			let is_splitter = line[i] == b'^';
			let is_active = bitmask[i];
			let is_split: bool = is_splitter & is_active;
			number_of_splits += u64::from(is_split);
			bitmask[i - 1] |= is_split;
			bitmask[i] &= !is_split;
			bitmask[i] |= delayed_store;
			delayed_store = is_split;
		}
	}
	number_of_splits
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
	let mut timelines = [0u64; 1024];
	let mut lines = input.lines();
	let first_line = lines.next().unwrap();
	let first_line = first_line.as_bytes();
	// Adding one to simplify logic, and rounding up to the nearest multiple of 8.
	let n = (first_line.len() / 8 + 1) * 8;
	let timelines = &mut timelines[..n];
	let start_pos = first_line.iter().copied().position(|x| x == b'S').unwrap();
	timelines[start_pos] = 1;
	for line in lines {
		let line = line.as_bytes();
		let k = n.min(line.len());
		let mut delayed_store = 0;
		for i in 1..k {
			let is_splitter = line[i] == b'^';
			let split_mask = if is_splitter { u64::MAX } else { 0 };
			let split_timelines: u64 = timelines[i] & split_mask;
			timelines[i - 1] += split_timelines;
			timelines[i] -= split_timelines;
			timelines[i] += delayed_store;
			delayed_store = split_timelines;
		}
	}
	timelines.iter().sum()
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

	const GIVEN: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

	#[test]
	fn test_part1_given() {
		init_logger();

		assert_eq!(part1(GIVEN), 21);
	}

	#[test]
	fn test_part2_given() {
		init_logger();

		assert_eq!(part2(GIVEN), 40);
	}
}
