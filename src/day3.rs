#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
	let solution = Solver::<2>::solve(input);
	debug_assert_eq!(solution, part1_clean(input));
	solution
}

fn part1_clean(input: &str) -> u64 {
	input
		.lines()
		.filter(|x| !x.is_empty())
		.map(find_largest_safe_joltage_in_bank)
		.sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
	let solution = Solver::<12>::solve(input);
	debug_assert_eq!(solution, part2_clean(input));
	solution
}

fn part2_clean(input: &str) -> u64 {
	input
		.lines()
		.filter(|x| !x.is_empty())
		.map(find_largest_overcharged_joltage_in_bank)
		.sum()
}

fn find_largest_safe_joltage_in_bank(bank: &str) -> u64 {
	let digits = bank.as_bytes();
	assert!(digits.len() >= 2);
	debug_assert!(digits.iter().all(|x| x.is_ascii_digit()));
	let mut start = 0;
	let mut first_digit = digits[0];
	for i in 1..(digits.len() - 1) {
		if digits[i] > first_digit {
			first_digit = digits[i];
			start = i;
		}
	}
	debug_assert!(start + 1 < digits.len());
	let second_digit = digits[(start + 1)..].iter().copied().max().unwrap();
	u64::from(first_digit - b'0') * 10 + u64::from(second_digit - b'0')
}

fn find_largest_overcharged_joltage_in_bank(bank: &str) -> u64 {
	let digits = bank.as_bytes();
	let mut leeway = digits.len() - 12;
	let mut joltage = 0;
	let mut start = 0;
	for _ in 0..12 {
		let mut best_digit = digits[start];
		let mut digits_skipped = 0;
		for skip in 1..=leeway {
			let i = start + skip;
			if digits[i] > best_digit {
				best_digit = digits[i];
				digits_skipped = skip;
			}
		}
		start += 1 + digits_skipped;
		leeway -= digits_skipped;
		joltage *= 10;
		joltage += u64::from(best_digit - b'0');
	}
	joltage
}

#[derive(Clone, Copy, Debug)]
struct Solver<const N: usize> {
	row: [u64; N],
	sum: u64,
}

impl<const N: usize> Solver<N> {
	fn solve(input: &str) -> u64 {
		let mut solver = Solver {
			row: [0; N],
			sum: 0,
		};
		for x in input.bytes() {
			solver.process(x);
		}
		solver.finalize()
	}

	fn process(&mut self, ascii_digit_or_whitespace: u8) {
		debug_assert!(b"\r\n\t 0123456789".contains(&ascii_digit_or_whitespace));
		let is_digit = (ascii_digit_or_whitespace & 0b00010000) != 0;
		let digit_value = u64::from(ascii_digit_or_whitespace & 0b00001111);

		let break_mask = if is_digit { 0u64 } else { u64::MAX };
		self.sum += break_mask & self.row[0];

		let mut candidates = [0; N];
		for i in 0..(N - 1) {
			candidates[i] = self.row[i + 1];
		}
		for i in 0..N {
			candidates[i] *= 10;
		}
		for i in 0..N {
			candidates[i] += digit_value;
		}
		for i in 0..N {
			self.row[i] = self.row[i].max(candidates[i]);
		}
		for i in 0..N {
			self.row[i] = self.row[i] & !break_mask;
		}
	}

	fn finalize(self) -> u64 {
		self.sum + self.row[0]
	}
}

#[cfg(test)]
mod tests {
	use pretty_assertions::assert_eq;

	use super::*;

	#[test]
	fn test_part1_given() {
		let given = "987654321111111
811111111111119
234234234234278
818181911112111";
		assert_eq!(part1(given), 357);
	}

	#[test]
	fn test_part2_given() {
		let given = "987654321111111
811111111111119
234234234234278
818181911112111";
		assert_eq!(part2(given), 3121910778619);
	}

	#[test]
	fn test_part1_testcases() {
		assert_eq!(part1("12"), 12);
		assert_eq!(part1("123"), 23);
		assert_eq!(part1("4341"), 44);
	}

	#[test]
	fn test_part2_testcases() {
		assert_eq!(part2("234234234234278"), 434234234278);
		assert_eq!(part2("281111111111119"), 811111111119);
		assert_eq!(part2("281111811111119"), 811811111119);
		assert_eq!(part2("282222811111119"), 822811111119);
		assert_eq!(part2("282342811111119"), 842811111119);
		assert_eq!(part2("822942811111119"), 942811111119);
		assert_eq!(part2("818888818888818"), 888888888888);
	}
}
