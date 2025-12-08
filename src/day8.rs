use parse_display::{Display, FromStr};

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u64 {
	solve_part1(input, 1000)
}

pub fn solve_part1(input: &str, num_connections: usize) -> u64 {
	let mut xs: Vec<i32> = Vec::with_capacity(1024);
	let mut ys: Vec<i32> = Vec::with_capacity(1024);
	let mut zs: Vec<i32> = Vec::with_capacity(1024);
	let mut cs: Vec<u32> = Vec::with_capacity(1024);
	for line in input.lines() {
		let point: Point = line.parse().unwrap();
		xs.push(point.x as i32);
		ys.push(point.y as i32);
		zs.push(point.z as i32);
		cs.push(0);
	}

	let mut connections: Vec<(i64, usize, usize)> = Vec::with_capacity(xs.len() * xs.len());
	let mut cutoff = i64::MAX;
	for i in 0..xs.len() {
		let x0 = xs[i];
		let y0 = ys[i];
		let z0 = zs[i];
		for j in 0..i {
			let dx = i64::from(xs[j] - x0);
			let dy = i64::from(ys[j] - y0);
			let dz = i64::from(zs[j] - z0);
			let squared_euclidean = dx * dx + dy * dy + dz * dz;
			if squared_euclidean > cutoff {
				continue;
			}
			connections.push((squared_euclidean, i, j));
		}
		if connections.len() >= 1000 {
			connections.sort();
			cutoff = connections[999].0;
			connections.drain(1000..);
		}
	}
	connections.sort();

	let mut next_unused_color = 1;
	let mut color_ids: Vec<Vec<usize>> = vec![Vec::new()];
	for (_dist, i, j) in connections.into_iter().take(num_connections) {
		match (cs[i], cs[j]) {
			(0, 0) => {
				let c = next_unused_color;
				next_unused_color += 1;
				color_ids.push(vec![i, j]);
				cs[i] = c;
				cs[j] = c;
			}
			(c, 0) => {
				color_ids[c as usize].push(j);
				cs[j] = c;
			}
			(0, c) => {
				color_ids[c as usize].push(i);
				cs[i] = c;
			}
			(c0, c1) if c0 == c1 => continue,
			(c0, c1) => {
				let consumed: Vec<usize> = std::mem::take(&mut color_ids[c1 as usize]);
				color_ids[c0 as usize].extend_from_slice(&consumed);
				for k in consumed {
					cs[k] = c0;
				}
			}
		}
	}

	let color_histogram: Vec<u32> = color_ids[1..].iter().map(|ids| ids.len() as u32).collect();
	let mut top_three = [1, 1, 1];
	for size in color_histogram {
		if size <= top_three[2] {
			continue;
		}
		let [a, b, _] = top_three;
		if size > a {
			top_three = [size, a, b];
		} else if size > b {
			top_three = [a, size, b];
		} else {
			top_three = [a, b, size];
		}
	}
	let [a, b, c] = top_three.map(u64::from);
	a * b * c
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
	let mut xs: Vec<i32> = Vec::with_capacity(1024);
	let mut ys: Vec<i32> = Vec::with_capacity(1024);
	let mut zs: Vec<i32> = Vec::with_capacity(1024);
	let mut cs: Vec<u32> = Vec::with_capacity(1024);
	for line in input.lines() {
		let point: Point = line.parse().unwrap();
		xs.push(point.x as i32);
		ys.push(point.y as i32);
		zs.push(point.z as i32);
		cs.push(0);
	}
	let num_boxes = xs.len();

	// Ouch.
	let mut connections: Vec<(i64, usize, usize)> = Vec::with_capacity(xs.len() * xs.len());
	for i in 0..xs.len() {
		let x0 = xs[i];
		let y0 = ys[i];
		let z0 = zs[i];
		for j in 0..i {
			let dx = i64::from(xs[j] - x0);
			let dy = i64::from(ys[j] - y0);
			let dz = i64::from(zs[j] - z0);
			let squared_euclidean = dx * dx + dy * dy + dz * dz;
			connections.push((squared_euclidean, i, j));
		}
	}
	connections.sort();

	let mut next_unused_color = 1;
	let mut color_ids: Vec<Vec<usize>> = vec![Vec::new()];
	for (_dist, i, j) in connections.into_iter() {
		let c = match (cs[i], cs[j]) {
			(0, 0) => {
				let c = next_unused_color;
				next_unused_color += 1;
				color_ids.push(vec![i, j]);
				c
			}
			(c, 0) => {
				color_ids[c as usize].push(j);
				c
			}
			(0, c) => {
				color_ids[c as usize].push(i);
				c
			}
			(c0, c1) if c0 == c1 => continue,
			(c0, c1) => {
				let consumed: Vec<usize> = std::mem::take(&mut color_ids[c1 as usize]);
				color_ids[c0 as usize].extend_from_slice(&consumed);
				for k in consumed {
					cs[k] = c0;
				}
				c0
			}
		};
		if color_ids[c as usize].len() >= num_boxes {
			return i64::from(xs[i]) * i64::from(xs[j]);
		}
		cs[i] = c;
		cs[j] = c;
	}
	unreachable!()
}

#[derive(Display, FromStr, Clone, Copy, Debug, Default, PartialEq)]
#[display("{x},{y},{z}")]
struct Point {
	x: u32,
	y: u32,
	z: u32,
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

	const GIVEN: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

	#[test]
	fn test_part1_given() {
		init_logger();

		assert_eq!(solve_part1(GIVEN, 10), 40);
	}

	#[test]
	fn test_part2_given() {
		init_logger();

		assert_eq!(part2(GIVEN), 25272);
	}
}
