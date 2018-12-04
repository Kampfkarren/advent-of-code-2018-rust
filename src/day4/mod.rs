use std::collections::HashMap;

type Number = u64;

#[derive(Default)]
struct Guard {
	minutes_asleep: HashMap<Number, Number>,
	minutes_slept: Number,
	id: Number,
}

impl Guard {
	fn new(id: Number) -> Guard {
		Guard {
			id,
			..Guard::default()
		}
	}
}

fn get_guards(lines: Vec<&str>) -> Vec<Guard> {
	let mut guards: HashMap<Number, Guard> = HashMap::new();
	let (mut current_guard, mut nap_time) = (0, 0);

	for line in lines {
		let mut iter = line.chars().skip("[YYYY-MM-DD HH:".len());
		let minute = {
			let mut string = String::new();
			string.push(iter.next().unwrap());
			string.push(iter.next().unwrap());
			string.parse::<Number>().unwrap()
		};

		iter.next(); iter.next();

		match iter.next().unwrap() {
			// Guard #XX begins shift
			'G' => {
				let mut iter = iter.skip("uard #".len());
				let mut id = String::new();
				loop {
					let character = iter.next().unwrap();
					if character.is_digit(10) {
						id.push(character);
					} else {
						break;
					}
				}

				let guard_id = id.parse::<Number>().unwrap();
				if guards.get(&guard_id).is_none() {
					guards.insert(guard_id, Guard::new(guard_id));
				}
				current_guard = guard_id;
			},

			// falls asleep
			'f' => {
				nap_time = minute;
			},

			'w' => {
				let minute = if minute < nap_time {
					// Napped at :58, woke up at :01, needs to set the max minutes to 61
					60 + minute
				} else {
					minute
				};

				let mut guard = &mut guards.get_mut(&current_guard).unwrap();

				for minute in nap_time..minute {
					*guard.minutes_asleep.entry(minute % 60).or_default() += 1;
					guard.minutes_slept += 1;
				}
			}

			character => panic!("Unkown character {}", character)
		};
	}

	guards.drain().map(|x| x.1).collect()
}

fn solution_part1(input: &str) -> Number {
	let mut lines = input.split('\n').collect::<Vec<&str>>();
	lines.sort_unstable();
	let guards = get_guards(lines);

	let sleepiest_guard = guards.iter().max_by_key(|x| x.minutes_slept).unwrap();
	let regularly_sleeps_at = sleepiest_guard.minutes_asleep.iter().max_by_key(|x| x.1).unwrap().0;
	sleepiest_guard.id * regularly_sleeps_at
}

fn solution_part2(input: &str) -> Number {
	#[derive(Default)]
	struct SleepStruct {
		how_long: Number,
		who: Number,
	}

	let mut lines = input.split('\n').collect::<Vec<&str>>();
	lines.sort_unstable();
	let guards = get_guards(lines);

	let mut sleeping_during_minutes: HashMap<Number, SleepStruct> = HashMap::new();
	for guard in guards {
		for (minute_asleep, how_long) in guard.minutes_asleep {
			let entry = sleeping_during_minutes.entry(minute_asleep).or_default();

			if how_long > entry.how_long {
				*entry = SleepStruct {
					how_long,
					who: guard.id,
				}
			}
		}
	}

	let (minute, sleep_struct) = sleeping_during_minutes.drain().max_by_key(|x| x.1.how_long).unwrap();
	minute * sleep_struct.who
}

pub fn solve(input: &str) {
	println!("Part 1: {}", solution_part1(input));
	println!("Part 2: {}", solution_part2(input));
}

#[test]
fn test_solution_part1() {
	assert_eq!(solution_part1(include_str!("./test.txt")), 240);

	// Times I messed up!
	assert!(solution_part1(include_str!("./input.txt")) > 36371);
}

#[test]
fn test_solution_part2() {
	assert_eq!(solution_part2(include_str!("./test.txt")), 4455);
}
