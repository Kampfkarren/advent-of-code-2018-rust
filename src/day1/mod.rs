fn get_numbers(input: &str) -> Vec<i32> {
	let mut numbers = Vec::new();

	for line in input.split('\n') {
		let mut chars = line.chars();
		let sign = match chars.next() {
			Some('+') => 1,
			Some('-') => -1,
			_ => unreachable!()
		};
		let number = chars.collect::<String>().parse::<i32>().unwrap();
		numbers.push(number * sign);
	}

	numbers
}

fn solution_part1(input: &str) -> i32 {
	get_numbers(input.into()).iter().sum()
}

fn solution_part2(input: &str) -> i32 {
	use std::collections::HashSet;

	let mut set: HashSet<i32> = HashSet::new();
	let numbers = get_numbers(input);
	let mut numbers = numbers.iter().cycle();
	let mut sum = numbers.next().unwrap().clone();

	for number in numbers {
		sum += number;
		if set.contains(&sum) {
			return sum;
		}
		set.insert(sum);
	}

	unreachable!();
}

pub fn solve(input: &str) {
	println!("Part 1: {}", solution_part1(input));
	println!("Part 2: {}", solution_part2(input));
}

#[test]
fn test_part1() {
	assert_eq!(solution_part1("+1\n+1\n+1"), 3);
	assert_eq!(solution_part1("+1\n+1\n-2"), 0);
	assert_eq!(solution_part1("-1\n-2\n-3"), -6);
}

#[test]
fn test_part2() {
	assert_eq!(solution_part2("+1\n-1"), 0);
	assert_eq!(solution_part2("+3\n+3\n+4\n-2\n-4"), 10);
	assert_eq!(solution_part2("-6\n+3\n+8\n+5\n-6"), 5);
	assert_eq!(solution_part2("+7\n+7\n-2\n-7\n-4"), 14);
}
