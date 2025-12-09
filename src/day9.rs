use parse_display::{Display, FromStr};

#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
	let mut xs = [0i32; 1024];
	let mut ys = [0i32; 1024];
	let n = {
		let mut i = 0;
		for line in input.lines() {
			let tile: Tile = line.parse().unwrap();
			xs[i] = tile.x;
			ys[i] = tile.y;
			i += 1;
		}
		i
	};
	let mut max_surface = 0;
	for i in 0..n {
		let x0 = xs[i];
		let y0 = ys[i];
		for j in 0..n {
			let dx = 1 + i64::from(xs[j] - x0).abs();
			let dy = 1 + i64::from(ys[j] - y0).abs();
			let surface = dx * dy;
			if surface > max_surface {
				max_surface = surface;
			}
		}
	}
	max_surface
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
	input.len() as i64 * 0
}

#[derive(Display, FromStr, Clone, Copy, Debug, PartialEq)]
#[display("{x},{y}")]
struct Tile {
	x: i32,
	y: i32,
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

	const GIVEN: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

	#[test]
	fn test_part1_given() {
		init_logger();

		assert_eq!(part1(GIVEN), 50);
	}

	#[test]
	fn test_part2_given() {
		init_logger();

		assert_eq!(part2(GIVEN), 0);
	}
}
