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
		if connections.len() >= num_connections {
			connections.sort();
			cutoff = connections[num_connections - 1].0;
			connections.drain(num_connections..);
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
	let mut xs = vec![0; 1024];
	let mut ys = vec![0; 1024];
	let mut zs = vec![0; 1024];
	let n = {
		let mut i = 0;
		for line in input.lines() {
			let point: Point = line.parse().unwrap();
			xs[i] = point.x as i32;
			ys[i] = point.y as i32;
			zs[i] = point.z as i32;
			i += 1;
		}
		i
	};

	let stride = (n / 32 + 1) * 32;
	assert!(stride <= 1024);
	assert!(n < stride);
	let mut edge_matrix = vec![0i64; n * stride];

	for i in 0..n {
		let x0 = xs[i];
		let y0 = ys[i];
		let z0 = zs[i];
		let edges = &mut edge_matrix[(i * stride)..(i * stride + stride)];
		for j in 0..stride {
			let dx = i64::from(xs[j] - x0);
			let dy = i64::from(ys[j] - y0);
			let dz = i64::from(zs[j] - z0);
			let squared_euclidean = dx * dx + dy * dy + dz * dz;
			edges[j] = squared_euclidean;
		}
		edges[i] = i64::MAX;
	}

	let mut min_cost = vec![i64::MAX; n];
	let mut min_edge_with = vec![0; n];
	min_cost[0] = 1;
	while let Some(i) = position_of_smallest_positive(&min_cost) {
		min_cost[i] = 0;
		for j in 0..n {
			let is_shortcut = edge_matrix[i * stride + j] < min_cost[j];
			let is_unexplored = min_cost[j] > 0;
			if is_shortcut {
				debug_assert!(is_unexplored);
				min_cost[j] = edge_matrix[i * stride + j];
				min_edge_with[j] = i;
			}
		}
	}
	let mut max_edge = 0;
	let mut max_edge_i = 0;
	let mut max_edge_j = 0;
	for i in 1..n {
		let j = min_edge_with[i];
		debug_assert_ne!(i, j);
		if edge_matrix[i * stride + j] > max_edge {
			max_edge = edge_matrix[i * stride + j];
			max_edge_i = i;
			max_edge_j = j;
		}
	}
	i64::from(xs[max_edge_i]) * i64::from(xs[max_edge_j])
}

fn position_of_smallest_positive(min_cost: &[i64]) -> Option<usize> {
	let mut best_i = None;
	let mut best_cost = i64::MAX;
	for i in 0..min_cost.len() {
		if min_cost[i] > 0 && min_cost[i] < best_cost {
			best_cost = min_cost[i];
			best_i = Some(i);
		}
	}
	best_i
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

	#[test]
	fn test_part2_testcase() {
		let text = "1,3,1\n5,1,1\n6,2,1\n10,9,1\n7,32,1";
		assert_eq!(part2(text), 10 * 7);
	}
}
