fn get_counts(line: &str) -> (bool, bool) {
	use std::collections::HashMap;
	let mut counts: HashMap<char, u8> = HashMap::new();

	for character in line.chars() {
		*counts.entry(character).or_default() += 1;
	}

	let (mut twos, mut threes) = (false, false);

	for value in counts.values() {
		let value = *value;
		if value == 2 {
			twos = true;
		} else if value == 3 {
			threes = true;
		}
	}

	(twos, threes)
}

fn solution_part1(input: &str) -> i32 {
	let (mut twos, mut threes) = (0, 0);

	for line in input.split('\n') {
		let (this_twos, this_threes) = get_counts(line);

		if this_twos {
			twos += 1;
		}

		if this_threes {
			threes += 1;
		}
	}

	twos * threes
}

fn solution_part2(input: &str) -> String {
	let lines = input.split('\n');

	for (index, line) in lines.clone().enumerate() {
		'this_line: for other_line in lines.clone().skip(index + 1) {
			let mut off_by_one = false;
			let mut result = String::new();

			for iterator in line.chars().zip(other_line.chars()) {
				let (char1, char2) = iterator;
				if char1 != char2 {
					if off_by_one {
						continue 'this_line;
					} else {
						off_by_one = true;
					}
				} else {
					result.push(char1);
				}
			}

			if off_by_one {
				return result;
			}
		}
	}

	unreachable!()
}

pub fn solve(input: &str) {
	println!("Part 1: {}", solution_part1(input));
	println!("Part 2: {}", solution_part2(input));
}

#[test]
fn test_get_counts() {
	assert_eq!(get_counts("abcdef"), (false, false));
	assert_eq!(get_counts("bababc"), (true, true));
	assert_eq!(get_counts("abcccd"), (false, true));
	assert_eq!(get_counts("abccccd"), (false, false));
}

#[test]
fn test_solution_part1() {
	assert_eq!(solution_part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 12);
}

#[test]
fn test_solution_part2() {
	assert_eq!(solution_part2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"), "fgij");
}
