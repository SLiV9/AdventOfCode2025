#[aoc(day6, part1)]
pub fn part1(input: &str) -> u64 {
	let answer = part1_second_approach(input);
	debug_assert_eq!(answer, part1_first_approach(input));
	answer
}

pub fn part1_first_approach(input: &str) -> u64 {
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
				b'+' | _ => {
					debug_assert_eq!(operator, b'+');
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

pub fn part1_second_approach(input: &str) -> u64 {
	let mut lines: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();
	let last_line = lines.pop().unwrap();
	let is_mul: Vec<u8> = last_line
		.bytes()
		.filter_map(|x| match x {
			b'+' => Some(0),
			b'*' => Some(1),
			_ => None,
		})
		.collect();
	let n = is_mul.len();

	let parse_number_line = |line: &str| {
		let mut numbers = vec![0u64; n];
		let mut i = 0;
		let mut number = 0;
		for x in line.as_bytes() {
			match x {
				b' ' => {
					if number > 0 {
						numbers[i] = number;
						i += 1;
						number = 0;
					}
				}
				_ => {
					debug_assert!((b'0'..=b'9').contains(&x));
					number *= 10;
					number += u64::from(x - b'0');
				}
			}
		}
		if i < n {
			numbers[i] = number;
			i += 1;
		}
		assert_eq!(i, n);
		numbers
	};

	let lines: Vec<Vec<u64>> = lines.into_iter().map(parse_number_line).collect();
	let mut lines = lines.into_iter();
	let a = lines.next().unwrap();
	let mut adds = a.clone();
	let mut muls = a;
	assert_eq!(adds.len(), n);
	assert_eq!(muls.len(), n);
	while let Some(b) = lines.next() {
		assert_eq!(b.len(), n);
		for i in 0..n {
			adds[i] += b[i];
			muls[i] *= b[i];
		}
	}

	let mut grand_total = 0;
	for i in 0..n {
		let is_mul = u64::from(is_mul[i]);
		grand_total += if is_mul > 0 { muls[i] } else { adds[i] };
	}
	grand_total
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u64 {
	let mut lines: Vec<&[u8]> = input
		.lines()
		.filter(|x| !x.is_empty())
		.map(|x| x.as_bytes())
		.collect();
	let last_line = lines.pop().unwrap();
	let mut grand_total = 0;
	let mut i = last_line.len();
	for line in &lines {
		assert_eq!(line.len(), last_line.len());
	}
	let mut stack = [0u64; 16];
	let mut stack_depth: usize = 0;
	while i > 0 {
		i -= 1;
		let mut number = 0;
		for line in &lines {
			let x = line[i];
			if x == b' ' {
				continue;
			};
			debug_assert!((b'0'..=b'9').contains(&x));
			number *= 10;
			number += u64::from(x - b'0');
		}
		stack[stack_depth] = number;
		stack_depth += 1;
		match last_line[i] {
			b'+' => {
				let sum: u64 = stack[..stack_depth].into_iter().sum();
				grand_total += sum;
				stack_depth = 0;
			}
			b'*' => {
				let product: u64 = stack[..stack_depth].into_iter().product();
				grand_total += product;
				stack_depth = 0;
			}
			b' ' | _ => {
				debug_assert_eq!(last_line[i], b' ');
				if number == 0 {
					stack_depth = 0;
				}
			}
		}
	}
	grand_total
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

		let given = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
		assert_eq!(part2(given), 3263827);
	}

	#[test]
	fn test_part2_testcases() {
		assert_eq!(part2("64 \n23 \n314\n+  "), 1058);
	}
}
