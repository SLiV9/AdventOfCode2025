use smallvec::SmallVec;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
	input.lines().map(str::trim).map(assess_part1).sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
	input.len()
}

fn assess_part1(line: &str) -> usize {
	let mut target: u32 = 0;
	let mut buttons: SmallVec<[u32; 16]> = SmallVec::new();
	{
		let bytes = line.as_bytes();
		let mut i = 1;
		let mut target_setter = 1;
		loop {
			match bytes[i] {
				b'#' => {
					target |= target_setter;
					target_setter <<= 1;
				}
				b'.' => {
					target_setter <<= 1;
				}
				b']' => {
					i += 3;
					break;
				}
				_ => (),
			}
			i += 1;
		}
		let mut button = 0;
		let mut number = 0;
		loop {
			let b = bytes[i];
			i += 1;
			match b {
				b'0'..=b'9' => {
					number *= 10;
					number += u32::from(b - b'0');
				}
				b',' => {
					button |= 0b1 << number;
					number = 0;
				}
				b')' => {
					button |= 0b1 << number;
					number = 0;
					buttons.push(button);
					button = 0;
				}
				b'{' => break,
				_ => continue,
			}
		}
	}

	let target: u32 = target;
	let buttons: &[u32] = &buttons[..];

	// dbg!(format!("{target:08b}"));
	// for &button in buttons {
	// 	dbg!(format!("{button:08b}"));
	// }

	let mut min_presses = u32::MAX;
	let num_possibilities = 2u32 << buttons.len();
	for possibility in 0..num_possibilities {
		// dbg!(format!("{possibility:016b}"));
		let num_pressed = possibility.count_ones();
		if num_pressed < min_presses {
			let mut result = 0;
			let mut possibility_bits = possibility;
			for &button in buttons {
				if possibility_bits & 0b1 != 0 {
					result ^= button;
				}
				possibility_bits >>= 1;
			}
			// dbg!(format!("{result:08b}"));
			if result == target {
				min_presses = num_pressed;
			}
		}
	}
	min_presses as usize
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

		assert_eq!(
			assess_part1("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
			2
		);
		assert_eq!(
			assess_part1("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
			3
		);
		assert_eq!(
			assess_part1("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"),
			2
		);
	}

	#[test]
	fn test_part2_given() {
		init_logger();

		let given = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
		assert_eq!(part2(given), 24);
	}
}
