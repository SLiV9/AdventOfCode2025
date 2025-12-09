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

	let rect_intersects_any_line = |x0: i32, y0: i32, x1: i32, y1: i32| {
		let (x0, x1) = (x0.min(x1), x0.max(x1));
		let (y0, y1) = (y0.min(y1), y0.max(y1));
		let mut x2 = xs[n - 1];
		let mut y2 = ys[n - 1];
		for k in 0..n {
			let x3 = xs[k];
			let y3 = ys[k];
			{
				let (x2, x3) = (x2.min(x3), x2.max(x3));
				let (y2, y3) = (y2.min(y3), y2.max(y3));
				if rect_intersects_line(x0, y0, x1, y1, x2, y2, x3, y3) {
					return true;
				}
			}
			x2 = x3;
			y2 = y3;
		}
		false
	};

	let mut max_surface = 0;
	for i in 0..n {
		let x0 = xs[i];
		let y0 = ys[i];
		for j in 0..i {
			let x1 = xs[j];
			let y1 = ys[j];
			let dx = 1 + i64::from(x1 - x0).abs();
			let dy = 1 + i64::from(y1 - y0).abs();
			let surface = dx * dy;
			if surface <= max_surface {
				continue;
			}
			// Not sure if this is right but if it works, it works.
			if rect_intersects_any_line(x0, y0, x1, y1) {
				continue;
			}
			max_surface = surface;
		}
	}
	max_surface
}

fn rect_intersects_line(
	x0: i32,
	y0: i32,
	x1: i32,
	y1: i32,
	x2: i32,
	y2: i32,
	x3: i32,
	y3: i32,
) -> bool {
	if x2 == x3 {
		// Vertical line.
		x0 < x2 && x2 < x1 && y3 >= y0 && y2 <= y1
	} else {
		// Horizontal line.
		debug_assert_eq!(y2, y3);
		y0 < y2 && y2 < y1 && x3 >= x0 && x2 <= x1
	}
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

		assert_eq!(part2(GIVEN), 24);
	}
}
