use std::io::Read;

fn main() {
	let mut stdin = std::io::stdin();
	let mut buf = String::new();
	stdin.read_to_string(&mut buf).unwrap();
	let out = solve(&buf);
	println!("{out}");
}

fn solve(input: &str) -> String {
	let mut lines = input
		.lines()
		.map(|l| {
			l.split_whitespace()
				.map(|x| x.parse::<i64>().unwrap())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	let mut numbers = lines.pop().unwrap();
	numbers.sort_unstable();

	let n = lines[0][0];
	let k = lines[0][1];

	let unseen = n * 2 - k;

	let mut last = i64::MIN;
	let mut pairs = 0;
	for number in numbers.into_iter() {
		if number == last {
			last = i64::MIN;
			pairs += 1;
		} else {
			last = number;
		}
	}

	pairs += unseen % n;

	format!("{pairs}")
}

#[cfg(test)]
mod test {
	use crate::*;

	#[test]
	fn test1() {
		let test = r"9 5
2 2 4 1 1";
		assert_eq!(solve(test), "2");
	}

	#[test]
	fn test2() {
		let test = r"2 3
1 2 1";
		assert_eq!(solve(test), "2");
	}
}
