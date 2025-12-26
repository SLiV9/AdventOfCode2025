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
	assert!(buttons.len() <= 16);
	assert!(joltage.len() <= 16);
	const DEPTH: usize = 1 << 16;
	let end = 1 << buttons.len();
	let all = end - 1;

	for &button in buttons {
		debug_print("button", button);
	}
	for &jolt in joltage {
		debug_print("joltage", jolt);
	}

	let mut equations: SmallVec<[u32; 16]> = SmallVec::new();
	equations.resize(joltage.len(), 0);
	{
		for i in 0..joltage.len() {
			for c in 0..buttons.len() {
				if buttons[c] & (1 << i) != 0 {
					equations[i] |= 1 << c;
				}
			}
		}
	}
	let equations: &[u32] = &equations[..];
	assert_eq!(joltage.len(), equations.len());

	for &equation in equations {
		debug_print("equation", equation);
	}

	let max_joltage: u32 = joltage.iter().sum();

	let mut lowers = [0; DEPTH];
	let mut uppers = [max_joltage; DEPTH];
	uppers[max_joltage as usize] = 0;

	for i in 0..joltage.len() {
		lowers[equations[i] as usize] = joltage[i];
		uppers[equations[i] as usize] = joltage[i];
	}

	let improve_lower = |candidate: u32, value: &mut u32, progress: &mut bool| {
		let is_improvement = candidate > *value;
		if is_improvement {
			*value = candidate;
		}
		*progress |= is_improvement;
	};
	let improve_upper = |candidate: u32, value: &mut u32, progress: &mut bool| {
		let is_improvement = candidate < *value;
		if is_improvement {
			*value = candidate;
		}
		*progress |= is_improvement;
	};

	loop {
		let mut progress = false;
		for a in 1..end {
			for b in 1..a {
				if a | b == b {
					let c = b & !a;
					debug_assert!(lowers[b] >= uppers[a]);
					improve_lower(lowers[b] - uppers[a], &mut lowers[c], &mut progress);
					debug_assert!(uppers[b] >= lowers[a]);
					improve_upper(uppers[b] - lowers[a], &mut uppers[c], &mut progress);
				} else {
					if a & b == 0 {
						improve_lower(lowers[a] + lowers[b], &mut lowers[a | b], &mut progress);
					}
					improve_upper(uppers[a] + uppers[b], &mut uppers[a | b], &mut progress);
				}
			}
		}
		if !progress {
			break;
		}
	}

	// if cfg!(debug_assertions) {
	// 	for i in 0..joltage.len() {}
	// }

	lowers[all] as usize
}

#[track_caller]
fn debug_print(name: &str, value: u32) {
	if cfg!(debug_assertions) {
		println!("[{}:{}] {name} = {value:08b} ({value})", file!(), line!());
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

pub fn is_correct(matrix: &[[u32; 16]; 16], vector: &[u32; 16], answer: &[u32; 16]) -> bool {
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
