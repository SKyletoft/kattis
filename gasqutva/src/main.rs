use std::{
	collections::{HashMap, VecDeque},
	io::Read,
};

fn main() {
	let mut stdin = std::io::stdin();
	let mut buf = String::new();
	stdin.read_to_string(&mut buf).unwrap();
	let out = solve(&buf);
	println!("{out}");
}

fn solve(input: &str) -> String {
	let lines = input
		.lines()
		.map(|l| {
			l.split_whitespace()
				.map(|x| x.parse::<usize>().unwrap())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	let student_count = lines[0][0];
	let students = lines
		.iter()
		.skip(1)
		.take(student_count)
		.map(|v| v[0])
		.collect::<Vec<_>>();
	let connections = {
		let mut connections = HashMap::<usize, Vec<usize>>::new();
		for (a, b) in lines
			.iter()
			.skip(1 + student_count)
			.map(|v| (v[0], v[1]))
		{
			connections
				.entry(a)
				.and_modify(|v| v.push(b))
				.or_insert_with(|| vec![b]);
			connections
				.entry(b)
				.and_modify(|v| v.push(a))
				.or_insert_with(|| vec![a]);
		}

		connections
	};

	let max = (1..=student_count)
		.map(|i| (i, calc_val(i, &students, &connections)))
		.max_by_key(|(_, x)| *x)
		.map(|(x, _)| x)
		.unwrap();

	format!("{max}")
}

fn calc_val(start: usize, generosity: &[usize], connections: &HashMap<usize, Vec<usize>>) -> usize {
	let mut donations = HashMap::<usize, usize>::new();
	donations.insert(start, 100);
	let mut queue = connections[&start]
		.iter()
		.copied()
		.map(|a| (start, a))
		.collect::<VecDeque<(usize, usize)>>();

	while let Some((friend, idx)) = queue.pop_front() {
		if donations.contains_key(&idx) {
			continue;
		}
		donations.insert(idx, generosity[idx - 1] + donations[&friend]);

		for connection in connections[&idx].iter()
			// .filter(|x| !donations.contains_key(x))
		{
			queue.push_back((idx, *connection));
		}
	}

	donations.values().copied().sum()
}

#[cfg(test)]
mod test {
	use crate::*;

	#[test]
	fn test1() {
		let test = r"3
50
60
100
2 1
1 3";
		assert_eq!(solve(test), "2");
	}
}
