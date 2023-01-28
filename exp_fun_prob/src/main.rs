use std::{
	collections::{HashMap, VecDeque},
	io::Read,
};

use primes::PrimeGenerator;

fn main() {
	let mut stdin = std::io::stdin();
	let mut buf = String::new();
	stdin.read_to_string(&mut buf).unwrap();
	let out = solve(&buf);
	println!("{out}");
}

fn solve(input: &str) -> String {
	let input = input.parse::<u32>().unwrap();
	let components = primes::PrimeGenerator::default()
		.iter()
		.take_while(|&&x| x <= input)
		.copied()
		.collect::<Vec<_>>();

	format!("")
}

fn naive(primes: &[u32], target: u32) -> u32 {
	if primes.iter().rev().any(|&x| x == target) {
		return target;
	}
	let sqrt = (target as f64).sqrt().ceil() as u32;
	
	todo!()
}

mod primes {
	use std::ops::Deref;

	const LAST_PRIME: u32 = 4_294_967_291; //Largest prime that fits in a u32, u32::MAX - 5

	#[derive(Debug)]
	pub struct PrimeGenerator {
		primes: Vec<u32>,
	}

	impl Deref for PrimeGenerator {
		type Target = [u32];

		fn deref(&self) -> &Self::Target {
			&self.primes
		}
	}

	impl Default for PrimeGenerator {
		fn default() -> Self {
			PrimeGenerator { primes: vec![1] }
		}
	}

	impl Iterator for PrimeGenerator {
		type Item = u32;

		fn next(&mut self) -> Option<Self::Item> {
			if self.primes[self.primes.len() - 1] == LAST_PRIME
				|| self.primes.len() == std::usize::MAX
			{
				return None;
			}
			((self.primes[self.primes.len() - 1])..=LAST_PRIME)
				.step_by(2)
				.skip(1)
				.find(|&candidate| {
					let limit = (candidate as f64).sqrt() as u32;
					self.primes
						.iter()
						.skip(1)
						.take_while(|&&p| p <= limit)
						.all(|prime| candidate % prime != 0)
				})
				.map(|c| {
					self.primes.push(c);
					c
				})
		}
	}
}

#[cfg(test)]
mod test {
	use crate::*;

	#[test]
	fn test1() {
		assert_eq!(solve("7"), "7 12");
	}
}
