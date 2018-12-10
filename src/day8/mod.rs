type Number = u64;

#[derive(Debug, Eq, PartialEq)]
struct Node {
	children: Vec<Box<Node>>,
	metadata: Vec<Number>,
}

impl Node {
	fn descendants(&self) -> Vec<&Node> {
		let mut descendants = vec![self];

		for child in &self.children {
			descendants.append(&mut child.descendants());
		}

		descendants
	}

	fn value(&self) -> Number {
		if self.children.is_empty() {
			self.metadata.iter().sum()
		} else {
			self.metadata.iter().map(|index| {
				if let Some(reference) = self.children.get((index - 1) as usize) {
					reference.value()
				} else {
					0
				}
			}).sum()
		}
	}
}

fn read_node<I: Iterator<Item=Number>>(numbers: &mut I) -> Option<Node> {
	let children_count = numbers.next()?;
	let metadata_entries = numbers.next().unwrap();
	let node = Node {
		children: std::iter::repeat_with(|| Box::new(read_node(numbers).unwrap())).take(children_count as usize).collect(),
		metadata: std::iter::repeat_with(|| numbers.next().unwrap()).take(metadata_entries as usize).collect(),
	};
	Some(node)
}

fn parse(input: &str) -> Node {
	read_node(&mut input.split(' ').map(|x| x.parse::<Number>().unwrap())).unwrap()
}

fn solution_part1(input: &str) -> Number {
	parse(input).descendants().iter().map(|node| &node.metadata).flatten().sum()
}

fn solution_part2(input: &str) -> Number {
	parse(input).value()
}

pub fn solve(input: &str) {
	println!("Part 1: {}", solution_part1(input));
	println!("Part 2: {}", solution_part2(input));
}

#[cfg(test)]
const TEST_DATA: &'static str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

#[test]
fn test_parse() {
	assert_eq!(parse(TEST_DATA), Node {
		children: vec![Box::new(Node {
			children: vec![],
			metadata: vec![10, 11, 12],
		}), Box::new(Node {
			children: vec![Box::new(Node {
				children: vec![],
				metadata: vec![99]
			})],
			metadata: vec![2],
		})],
		metadata: vec![1, 1, 2],
	});
}

#[test]
fn test_solution_part1() {
	assert_eq!(solution_part1(TEST_DATA), 138);
}

#[test]
fn test_solution_part2() {
	assert_eq!(solution_part2(TEST_DATA), 66);
}
