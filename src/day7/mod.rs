#![allow(clippy::cast_lossless)]
#![allow(clippy::char_lit_as_u8)]

// Me not good at Rust :P
use std::collections::{HashMap, HashSet};

type Number = u64;

#[derive(Debug, Default, Eq, PartialEq)]
struct Node {
	id: char,
	number: Number,
	children: HashSet<char>,
	parents: HashSet<char>,
}

impl Node {
	fn new(id: char) -> Node {
		Node {
			id,
			number: ((id as u8) - ('A' as u8) + 1) as Number,
			..Node::default()
		}
	}
}

fn parse(input: &str) -> Vec<(char, char)> {
	let mut result = Vec::new();

	for line in input.split('\n') {
		let mut chars = line.chars().skip("Step ".len());
		let first_char = chars.next().unwrap();
		result.push((first_char, chars.nth(" must be finished before step ".len()).unwrap()));
	}

	result
}

fn make_tree(input: Vec<(char, char)>) -> HashMap<char, Node> {
	let mut nodes: HashMap<char, Node> = HashMap::new();

	for (parent, child) in input {
		let parent_node = nodes.entry(parent).or_insert_with(|| Node::new(parent));
		parent_node.children.insert(child);

		let child_node = nodes.entry(child).or_insert_with(|| Node::new(child));
		child_node.parents.insert(parent);
	}

	nodes
}

fn sorted_hash_set(set: HashSet<char>) -> Vec<char> {
	let mut result = Vec::new();
	result.extend(set);
	result.sort_unstable_by_key(|x| *x);
	result
}

fn get_roots(nodes: &HashSet<char>, tree: &mut HashMap<char, Node>) -> Vec<char> {
	sorted_hash_set(nodes.iter().filter(|index| tree[index].parents.is_empty()).cloned().collect::<HashSet<_>>())
}

fn solution_part1(input: &str) -> String {
	let parsed = parse(input);
	let mut tree = make_tree(parsed);

	fn inner(tree: &mut HashMap<char, Node>, nodes: &HashSet<char>) -> String {
		let mut result = String::new();

		for root_index in get_roots(nodes, tree) {
			let root = tree.remove(&root_index).unwrap();
			result.push(root.id);
			for child in sorted_hash_set(root.children.clone()) {
				tree.get_mut(&child).unwrap().parents.remove(&root.id);
			}
			result += &inner(tree, &root.children);
		}

		result
	}

	let keys = tree.keys().cloned().collect();
	inner(&mut tree, &keys)
}

fn solution_part2(input: &str, workers: usize, difference: Number) -> Number {
	let mut tree = make_tree(parse(input));
	for mut node in tree.values_mut() {
		node.number += difference;
	}

	let mut roots = get_roots(&tree.keys().cloned().collect(), &mut tree);
	let mut tasks = HashSet::with_capacity(workers);
	let mut sum = 0;

	while !roots.is_empty() {
		while tasks.len() < workers && tasks.len() < roots.len() {
			for root in &roots {
				if !tasks.contains(root) {
					tasks.insert(*root);
				}

				if tasks.len() >= workers || tasks.len() >= roots.len() {
					break;
				}
			}
		}

		sum += 1;

		let mut pending_removal = HashSet::new();

		for id in tasks.clone().iter() {
			let mut node = &mut tree.get_mut(id).unwrap();
			node.number -= 1;
			if node.number == 0 {
				pending_removal.insert(*id);
				tasks.remove(id);

				for child in node.children.clone() {
					let child_node = &mut tree.get_mut(&child).unwrap();
					child_node.parents.remove(id);
					if child_node.parents.is_empty() {
						roots.push(child);
					}
				}
			}
		}

		roots.retain(|&x| !pending_removal.contains(&x));
	}

	sum
}

pub fn solve(input: &str) {
	println!("Part 1: {}", solution_part1(input));
	println!("Part 2: {}", solution_part2(input, 5, 60));
}

#[test]
fn test_parse() {
	assert_eq!(parse(include_str!("./test.txt")), vec![
		('C', 'A'),
		('C', 'F'),
		('A', 'B'),
		('A', 'D'),
		('B', 'E'),
		('D', 'E'),
		('F', 'E'),
	]);
}

#[test]
fn test_make_tree() {
	let tree = make_tree(parse(include_str!("./test.txt")));

	macro_rules! set {
		() => { HashSet::new() };

		($($chars:tt),*) => {{
			let mut set = HashSet::new();
			$(set.insert($chars);)*
			set
		}};
	}

	let mut map = HashMap::new();

	map.insert('A', Node {
		id: 'A',
		number: 1,
		children: set!['B', 'D'],
		parents: set!['C'],
	});

	map.insert('B', Node {
		id: 'B',
		number: 2,
		children: set!['E'],
		parents: set!['A'],
	});

	map.insert('C', Node {
		id: 'C',
		number: 3,
		children: set!['A', 'F'],
		parents: set![],
	});

	map.insert('D', Node {
		id: 'D',
		number: 4,
		children: set!['E'],
		parents: set!['A'],
	});

	map.insert('E', Node {
		id: 'E',
		number: 5,
		children: set![],
		parents: set!['B', 'D', 'F'],
	});

	map.insert('F', Node {
		id: 'F',
		number: 6,
		children: set!['E'],
		parents: set!['C'],
	});
}

#[test]
fn test_solution_part1() {
	assert_eq!(solution_part1(include_str!("./test.txt")), "CABDFE");
}

#[test]
fn test_solution_part2() {
	assert_eq!(solution_part2(include_str!("./test.txt"), 2, 0), 15);
}
