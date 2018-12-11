use std::collections::{HashSet, HashMap};

type Coordinate = i32;
const RANGE: Coordinate = 100;

#[derive(Debug, Eq, PartialEq)]
struct Point {
	position: (Coordinate, Coordinate),
	velocity: (Coordinate, Coordinate),
}

struct StarPattern {
	points: Vec<Point>,
	time: u64,
}

impl StarPattern {
	fn from_string(input: &str) -> StarPattern {
		fn read_coordinate<I: Iterator<Item=char>>(iter: &mut std::iter::Peekable<I>) -> Coordinate {
			let mut string = String::new();
			while let Some(character) = iter.peek() {
				if *character != ',' && *character != '>' {
					string.push(iter.next().unwrap());
				} else {
					break;
				}
			}
			string.trim().parse::<Coordinate>().unwrap()
		}

		StarPattern {
			points: {
				let mut points = Vec::new();

				for line in input.lines() {
					let mut iter = line.chars().skip("position=<".len()).peekable();
					let position_x = read_coordinate(&mut iter);
					iter.next(); iter.next();
					let position_y = read_coordinate(&mut iter);
					let mut iter = iter.skip("> velocity=<".len()).peekable();
					let velocity_x = read_coordinate(&mut iter);
					iter.next(); iter.next();
					let velocity_y = read_coordinate(&mut iter);
					points.push(Point {
						position: (position_x, position_y),
						velocity: (velocity_x, velocity_y),
					});
				}

				points
			},

			time: 0,
		}
	}

	fn next(&mut self) -> Option<String> {
		self.time += 1;
		let (mut smallest_x, mut smallest_y) = (Coordinate::max_value(), Coordinate::max_value());
		let (mut biggest_x, mut biggest_y) = (0, 0);
		let mut grid: HashMap<Coordinate, HashSet<Coordinate>> = HashMap::new();

		for mut point in &mut self.points {
			point.position.0 += point.velocity.0;
			point.position.1 += point.velocity.1;
			grid.entry(point.position.1).or_default().insert(point.position.0);

			if smallest_x > point.position.0 {
				smallest_x = point.position.0;
			}

			if smallest_y > point.position.1 {
				smallest_y = point.position.1;
			}

			if biggest_x < point.position.0 {
				biggest_x = point.position.0;
			}

			if biggest_y < point.position.1 {
				biggest_y = point.position.1;
			}
		}

		if biggest_x - smallest_x > RANGE || biggest_y - smallest_y > RANGE {
			return None;
		}

		let mut result = String::new();

		for y in smallest_y..=biggest_y {
			if let Some(y_axis) = grid.get(&y) {
				for x in smallest_x..=biggest_x {
					if y_axis.contains(&x) {
						result.push('#');
					} else {
						result.push('.');
					}
				}
			} else {
				for _ in smallest_x..=biggest_x {
					result.push('.');
				}
			}

			result.push('\n');
		}

		Some(result)
	}
}

pub fn solve(input: &str) {
	let mut pattern = StarPattern::from_string(input);

	loop {
		if let Some(next) = pattern.next() {
			println!("{}\nTook {} seconds", next, pattern.time);
			std::io::stdin().read_line(&mut String::new()).unwrap();
		}
	}
}

#[test]
fn test_solution() {
	let mut pattern = StarPattern::from_string(include_str!("./test.txt"));
	pattern.next(); pattern.next();
	assert_eq!(pattern.next(), Some(include_str!("./test_message.txt").to_string()));
	assert_eq!(pattern.time, 3);
}

#[test]
fn test_patten_from_file() {
	let pattern = StarPattern::from_string(include_str!("./test.txt"));

	assert_eq!(pattern.points[0], Point {
		position: (9, 1),
		velocity: (0, 2),
	});

	assert_eq!(pattern.points[1], Point {
		position: (7, 0),
		velocity: (-1, 0),
	});
}
