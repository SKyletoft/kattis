use std::io::Read;

fn main() {
	let mut stdin = std::io::stdin();
	let mut buf = String::new();
	stdin.read_to_string(&mut buf).unwrap();
	let out = solve(&buf);
	println!("{out}");
}

fn parse(s: &str) -> (&str, i64, i64) {
	let mut split = s.split_whitespace();
	(
		split.next().unwrap(),
		split.next().unwrap().parse().unwrap(),
		split.next().unwrap().parse().unwrap(),
	)
}

fn solve(input: &str) -> String {
	let bars = input
		.lines()
		.skip(1)
		.collect::<Vec<_>>();
	let (n, t) = bars.into_iter()
		.rev()
		.map(parse)
		.map(|(n, a, b)| (n, (a + 1) * b))
		.max_by_key(|(_, x)| *x)
		.unwrap();
	format!("{n} {t}")
}

#[cfg(test)]
mod test {
	use crate::*;

	#[test]
	fn test1() {
		let test = r"3
Basen 10 5
Gasquen 20 5
JAPripps 5 20";
		assert_eq!(solve(test), "JAPripps 120");
	}

	#[test]
	fn test2() {
		let test = r"5
Basen 5 8
Focus 20 7
Gasqutva 13 21
Kajsabaren 9999 10
Hubben 22 11";
		assert_eq!(solve(test), "Kajsabaren 100000");
	}
}
