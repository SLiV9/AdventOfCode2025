use smallvec::SmallVec;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
	input.lines().map(str::trim).map(assess_part1).sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
	input.lines().map(str::trim).map(assess_part2).sum()
}

fn assess_part1(line: &str) -> usize {
	let mut target: u32 = 0;
	let mut buttons: SmallVec<[u32; 16]> = SmallVec::new();
	{
		let bytes = line.as_bytes();
		let mut i = 1;
		extract_target(&mut target, bytes, &mut i);
		extract_buttons(&mut buttons, bytes, &mut i);
	}

	let target: u32 = target;
	let buttons: &[u32] = &buttons[..];

	debug_print("target", target);
	for &button in buttons {
		debug_print("button", button);
	}

	let mut min_presses = u32::MAX;
	let num_possibilities = 2u32 << buttons.len();
	for possibility in 0..num_possibilities {
		debug_print("possibility", possibility);
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
			debug_print("result", result);
			if result == target {
				min_presses = num_pressed;
			}
		}
	}
	min_presses as usize
}

fn assess_part2(line: &str) -> usize {
	let mut buttons: SmallVec<[u32; 16]> = SmallVec::new();
	let mut joltage: SmallVec<[u32; 16]> = SmallVec::new();
	{
		let bytes = line.as_bytes();
		let mut i = 1;
		let mut target = 0;
		extract_target(&mut target, bytes, &mut i);
		extract_buttons(&mut buttons, bytes, &mut i);
		extract_joltage(&mut joltage, bytes, &mut i);
	}

	let buttons: &[u32] = &buttons[..];
	let joltage: &[u32] = &joltage[..];
	assert!(buttons.len() <= 15);
	assert!(joltage.len() <= 15);

	let mut answer = [0; 16];
	for r in 0..joltage.len() {
		answer[r] = joltage[r];
	}
	answer[15] = 0;

	let mut button_matrix = [[0u32; 16]; 16];
	for c in 0..buttons.len() {
		let mut button = buttons[c];
		let mut r = 0;
		while button != 0 {
			if button & 0b1 != 0 {
				button_matrix[r][c] = 1;
			}
			r += 1;
			button >>= 1;
		}
		button_matrix[15][c] = 1;
	}
	let button_matrix = button_matrix;

	let mut max_presses_per_button = [0; 16];
	for c in 0..buttons.len() {
		let mut min = u32::MAX;
		for r in 0..joltage.len() {
			let value = joltage[r];
			if value < min && button_matrix[r][c] != 0 {
				min = value;
			}
		}
		max_presses_per_button[c] = min;
	}
	let max_presses_per_button = max_presses_per_button;

	loop {
		answer[15] += 1;
		let presses = max_presses_per_button;
		// TODO iterate through all possible presses that sum up to answre[15]???
		if is_correct(&button_matrix, &presses, &answer) {
			return answer[15] as usize;
		}
		todo!()
	}
}

#[track_caller]
fn debug_print(name: &str, value: u32) {
	if cfg!(debug_assertions) {
		println!("[{}:{}] {name} = {value:08b}", file!(), line!());
	}
}

fn extract_target(target: &mut u32, bytes: &[u8], i: &mut usize) {
	let mut target_setter = 1;
	loop {
		match bytes[*i] {
			b'#' => {
				*target |= target_setter;
				target_setter <<= 1;
			}
			b'.' => {
				target_setter <<= 1;
			}
			b']' => {
				*i += 3;
				break;
			}
			_ => (),
		}
		*i += 1;
	}
}
fn extract_buttons(buttons: &mut SmallVec<[u32; 16]>, bytes: &[u8], i: &mut usize) {
	let mut button = 0;
	let mut number = 0;
	loop {
		let b = bytes[*i];
		*i += 1;
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
			b'{' => return,
			_ => continue,
		}
	}
}

fn extract_joltage(joltage: &mut SmallVec<[u32; 16]>, bytes: &[u8], i: &mut usize) {
	let mut number = 0;
	loop {
		let b = bytes[*i];
		*i += 1;
		match b {
			b'0'..=b'9' => {
				number *= 10;
				number += u32::from(b - b'0');
			}
			b',' => {
				joltage.push(number);
				number = 0;
			}
			b'}' => {
				joltage.push(number);
				return;
			}
			_ => continue,
		}
	}
}

fn is_correct(matrix: &[[u32; 16]; 16], vector: &[u32; 16], answer: &[u32; 16]) -> bool {
	let mut result = [0; 16];
	for r in 0..16 {
		for c in 0..16 {
			result[r] += matrix[r][c] * vector[c];
			if result[r] != answer[r] {
				return false;
			}
		}
	}
	true
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

		assert_eq!(
			assess_part2("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
			10
		);
		assert_eq!(
			assess_part2("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
			12
		);
		assert_eq!(
			assess_part2("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"),
			11
		);
	}
}
