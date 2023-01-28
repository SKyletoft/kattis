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
			let mut s = l.split_whitespace();
			(
				s.next().unwrap().parse::<i64>().unwrap(),
				s.next().unwrap().parse::<i64>().unwrap(),
			)
		})
		.collect::<Vec<_>>();
	let [(_student_count, _porridges), (mut a, mut b), students @ ..] = lines.as_slice() else { panic!() };

	let mut students = students.iter().copied().enumerate().map(|(i, (a, b))| (i, a, b)).collect::<Vec<_>>();
	students.sort_unstable_by_key(|(_, h, _)| *h);

	let mut pleased = 0;
	let mut displeased = 0;

	for (_, h_a, h_b) in students.iter().copied() {

	}


	format!("")
}

#[cfg(test)]
mod test {
	use crate::*;

	#[test]
	fn test1() {
		let test = r"3 2
2 1
4 5
2 1
4 2";

		assert_eq!(solve(test), "2");
	}
}
