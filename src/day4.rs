#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
	let mut floor = Floor::default();
	floor.fill(input);
	log::debug!("{floor}");
	let num_rolls_removed = floor.iterate();
	log::debug!("{floor}");
	num_rolls_removed
}

struct Floor {
	grid: [[u8; 256]; 256],
	n_rows: usize,
	n_cols: usize,
}

impl Default for Floor {
	fn default() -> Self {
		Floor {
			grid: [[0u8; 256]; 256],
			n_rows: 0,
			n_cols: 0,
		}
	}
}

impl std::fmt::Display for Floor {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in &self.grid[..(self.n_rows + 2)] {
			let cells = &row[..(self.n_cols + 2)];
			let line = std::str::from_utf8(cells).unwrap();
			f.write_str(line)?;
			f.write_str("\n")?;
		}
		Ok(())
	}
}

impl Floor {
	fn fill(&mut self, input: &str) {
		let mut r = 1;
		for line in input.lines().filter(|x| !x.is_empty()) {
			debug_assert!(self.n_cols == 0 || self.n_cols == line.len());
			self.n_cols = line.len();
			self.grid[r][1..(1 + line.len())].copy_from_slice(line.as_bytes());
			r += 1;
		}
		self.n_rows = r - 1;

		if cfg!(debug_assertions) {
			self.grid[0][..(self.n_cols + 2)].fill(b' ');
			for r0 in 0..self.n_rows {
				self.grid[r0 + 1][0] = b' ';
				self.grid[r0 + 1][self.n_cols + 1] = b' ';
			}
			self.grid[self.n_rows + 1][..(self.n_cols + 2)].fill(b' ');
		}
	}

	fn iterate(&mut self) -> usize {
		let mut num_changes = 0;
		assert!(self.n_rows + 3 <= 256);
		assert!(self.n_cols + 3 <= 256);
		for r in 0..self.n_rows {
			let (top_rows, other_rows) = self.grid.split_at_mut(r + 1);
			let top_row = &top_rows[r];
			let (mid_row, bot_rows) = other_rows.split_first_mut().unwrap();
			let bot_row = &bot_rows[0];
			for c in 0..self.n_cols {
				let top: &[u8; 3] = top_row[c..].first_chunk().unwrap();
				let mid: &mut [u8; 3] = mid_row[c..].first_chunk_mut().unwrap();
				let bot: &[u8; 3] = bot_row[c..].first_chunk().unwrap();
				num_changes += update_square(top, mid, bot)
			}
		}
		num_changes
	}
}

fn update_square(top: &[u8; 3], mid: &mut [u8; 3], bot: &[u8; 3]) -> usize {
	let num_surrounding = has_roll(top[0])
		+ has_roll(top[1])
		+ has_roll(top[2])
		+ has_roll(mid[0])
		+ has_roll(mid[2])
		+ has_roll(bot[0])
		+ has_roll(bot[1])
		+ has_roll(bot[2]);
	if num_surrounding < 4 && mid[1] == b'@' {
		mid[1] = b'x';
		1
	} else {
		0
	}
}

fn has_roll(cell: u8) -> u8 {
	u8::from(cell == b'@' || cell == b'x')
}

#[aoc(day4, part2)]
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

		let given = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
		assert_eq!(part1(given), 13);
	}

	#[test]
	fn test_part2_given() {
		init_logger();

		let given = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
		assert_eq!(part2(given), 3121910778619);
	}
}
