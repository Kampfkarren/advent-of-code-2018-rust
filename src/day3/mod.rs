use std::collections::HashMap;

type Number = u64;

#[derive(Debug, PartialEq)]
struct Plot {
	id: Number,
	left: Number,
	top: Number,
	width: Number,
	height: Number,
}

fn next_number<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) -> Result<Number, std::num::ParseIntError> {
	let mut string = String::new();

	while let Some(character) = chars.peek() {
		if character.to_digit(10).is_some() {
			string.push(chars.next().unwrap());
		} else {
			break;
		}
	}

	Ok(string.parse::<Number>()?)
}

fn parse_plot(line: &str) -> Plot {
	let mut chars = line.chars().peekable();
	chars.next(); // #
	let id = next_number(&mut chars).expect("Can't find ID");
	let mut chars = chars.skip(3).peekable(); // space @ space
	let left = next_number(&mut chars).expect("Can't find left");
	chars.next(); // ,
	let top = next_number(&mut chars).expect("Can't find top");
	let mut chars = chars.skip(2).peekable(); // : space
	let width = next_number(&mut chars).expect("Can't find width");
	chars.next(); // x
	let height = next_number(&mut chars).expect("Can't find height");

	Plot {
		id,
		left,
		top,
		width,
		height,
	}
}

fn get_plots(input: &str) -> Vec<Plot> {
	input.split('\n').map(parse_plot).collect()
}

fn get_overlap(input: Vec<Plot>) -> HashMap<(Number, Number), Vec<Number>> {
	let mut covered: HashMap<(Number, Number), Number> = HashMap::new();
	let mut overlap: HashMap<(Number, Number), Vec<Number>> = HashMap::new();

	for plot in input {
		for x in plot.left..(plot.left + plot.width) {
			for y in plot.top..(plot.top + plot.height) {
				let coords = (x, y);
				if let Some(original_id) = covered.get(&coords) {
					overlap.entry(coords).or_insert_with(|| vec![*original_id]).push(plot.id);
				} else {
					covered.insert(coords, plot.id);
				}
			}
		}
	}

	overlap
}

fn solution_part1(input: &str) -> Number {
	get_overlap(get_plots(input)).len() as Number
}

fn solution_part2(input: &str) -> Number {
	let plots = get_plots(input);
	let mut plot_ids = {
		let mut map: HashMap<Number, bool> = HashMap::new();
		for plot in &plots {
			map.insert(plot.id, false);
		}
		map
	};
	let overlap = get_overlap(plots);

	for offenders in overlap.values() {
		for offender in offenders {
			if plot_ids.get(offender).is_some() {
				plot_ids.remove(offender);
			}
		}
	}

	*plot_ids.keys().next().unwrap()
}

pub fn solve(input: &str) {
	println!("Part 1: {}", solution_part1(input));
	println!("Part 2: {}", solution_part2(input));
}

#[test]
fn test_next_number() {
	let text = "123 456,";
	let mut chars = text.chars().peekable();
	assert_eq!(next_number(&mut chars), Ok(123));
	chars.next();
	assert_eq!(next_number(&mut chars), Ok(456));
	assert_eq!(chars.next(), Some(','));
}

#[test]
fn test_parse_plot() {
	assert_eq!(parse_plot("#123 @ 3,2: 5x4"), Plot {
		id: 123,
		left: 3,
		top: 2,
		width: 5,
		height: 4,
	});
}

#[test]
fn test_solution_part1() {
	assert_eq!(solution_part1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 4);
}

#[test]
fn test_solution_part2() {
	assert_eq!(solution_part2("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 3);
}
