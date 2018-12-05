fn solution_part1(input: &str) -> u64 {
	let mut input = input.to_string();

	loop {
		let chars: Vec<char> = input.chars().collect();
		let mut index = 0;
		let mut new_input = String::new();
		let mut modded = false;

		while index < input.len() - 1 {
			let character = chars[index];
			let next_character = chars[index + 1];

			if character.to_ascii_lowercase() == next_character.to_ascii_lowercase() && character != next_character {
				modded = true;
				index += 1;
			} else {
				new_input.push(character);
			}

			index += 1;
		}

		new_input.push(*chars.last().unwrap());

		input = new_input;

		if !modded {
			break;
		}
	}

	input.len() as u64
}

fn solution_part2(input: &str) -> u64 {
	let mut chars: Vec<char> = input.to_ascii_lowercase().chars().collect();
	chars.sort();
	chars.dedup();
	let mut smallest = input.len() as u64;

	for character in chars {
		let result = solution_part1(&input.replace(character, "").replace(character.to_ascii_uppercase(), ""));
		if result < smallest {
			smallest = result;
		}
	}

	smallest
}

pub fn solve(input: &str) {
	println!("Part 1: {}", solution_part1(input));
	println!("Part 2: {}", solution_part2(input));
}

#[test]
fn test_solution_part1() {
	assert_eq!(solution_part1("dabAcCaCBAcCcaDA"), 10);
}

#[test]
fn test_solution_part2() {
	assert_eq!(solution_part2("dabAcCaCBAcCcaDA"), 4);
}
