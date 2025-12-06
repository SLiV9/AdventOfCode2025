#[aoc(day6, part1)]
pub fn part1(input: &str) -> u64 {
	let mut lines: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();
	let last_line = lines.pop().unwrap();
	let operators: Vec<u8> = last_line.bytes().filter(|x| b"+*".contains(x)).collect();
	let mut sum_of_adds = 0;
	let mut number_lines = lines.into_iter();
	let n_mul = operators.iter().filter(|&&x| x == b'*').count();

	let mut process_line = |line: &str, mul: &mut Vec<u64>| {
		let mut operators = operators.iter().copied();
		for number in line.split_ascii_whitespace() {
			if number.is_empty() {
				continue;
			}
			let number: u64 = number.parse().unwrap();
			let operator = operators.next().unwrap();
			match operator {
				b'*' => mul.push(number),
				b'+' => sum_of_adds += number,
				_ => {
					debug_assert!(false);
					sum_of_adds += number;
				}
			}
		}
		assert_eq!(mul.len(), n_mul);
	};

	let mut mul_a: Vec<u64> = Vec::with_capacity(n_mul);
	let mut mul_b: Vec<u64> = Vec::with_capacity(n_mul);
	process_line(number_lines.next().unwrap(), &mut mul_a);
	while let Some(number_line) = number_lines.next() {
		mul_b.clear();
		process_line(number_line, &mut mul_b);
		assert_eq!(mul_a.len(), mul_b.len());
		for i in 0..n_mul {
			mul_a[i] *= mul_b[i];
		}
	}
	let sum_of_muls: u64 = mul_a.into_iter().sum();
	sum_of_adds + sum_of_muls
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u64 {
	input.len() as u64
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

		let given = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
		assert_eq!(part1(given), 4277556);
	}

	#[test]
	fn test_part2_given() {
		init_logger();

		let given = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
		assert_eq!(part2(given), 0);
	}
}
