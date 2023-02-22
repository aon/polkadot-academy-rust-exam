//! This portion of the exam tests your abilities to work with iterators and their functional-style
//! methods.
//!
//! Throughout this portion of the test, you may refer to https://doc.rust-lang.org/std/iter/trait.Iterator.html
//! and other docs about iterators. You may NOT look up specific implementations for these problems
//! in Rust or any other Functional language.
//!
//! If you find that you simply cannot write these methods in the functional style using iterator
//! methods, writing them in the imperative style with loops will still earn partial credit.

/// This function takes an iterator of u32 values, squares each value, and returns the sum of the
/// squares. You may assume that no individual square, nor the entire sum, overflows the u32 type.
pub fn sum_of_squares(vals: impl Iterator<Item = u32>) -> u32 {
	let mut sum = 0;
	for val in vals {
		sum += val.pow(2);
	}
	sum
}

/// This function takes an iterator of i32 values, calculates the absolute value of each, and throws
/// away any values that are greater than 100. The remaining positive values are returned as an
/// iterator of u32s.
pub fn bounded_absolute_values(vals: impl Iterator<Item = i32>) -> impl Iterator<Item = u32> {
	// as u32 is (in this case) safe to use due to abs(i32) fitting inside u32
	vals.map(|x| x.abs() as u32).filter(|x| x <= &100)
}

// We allow `unused_mut` only so that there is no build warning on the starter code.
// You should remove this line once you have completed the following function
/// This function takes an iterator of u32 values. The first value in the iterator, call it n, is
/// special: it represents the maximum count of the resultant iterator. Once n is known, create an
/// iterator that yields the first n even values from the remainder of the input iterator.
///
/// If the input iterator is empty, return None
/// If there are fewer than n even values left in the input, return as many as possible
pub fn first_n_even(mut vals: impl Iterator<Item = u32>) -> Option<impl Iterator<Item = u32>> {
	let n = vals.next()?;
	// Although it'll probably never be ran in a platform with
	// 16-bit native integers (if such still exist), it's never too late
	// to panic in case u32 doesn't fit in usize
	let n_us = usize::try_from(n).unwrap();
	Some(vals.filter(|x| x % 2 == 0).take(n_us))
}

/// Return an "infinite" iterator that yields the squares of the whole numbers.
/// For example, the first few values should be 0, 1, 4, 9, 16, 25, ...
///
/// The iterator should be bounded only by the u32 type, not by your code
pub fn square_whole_numbers() -> impl Iterator<Item = u32> {
	(0..).map(|x: u32| x.pow(2))
}

/// An iterator that generates the Fibonacci sequence.
#[derive(Default)]
pub struct Fibonacci {
	/// The most recent value this iterator has yielded
	prev: Option<u32>,
	/// The second most recent value that this iterator has yielded
	prev_prev: Option<u32>,
}

impl Iterator for Fibonacci {
	type Item = u32;

	fn next(&mut self) -> Option<u32> {
		match (self.prev_prev, self.prev) {
			(Some(prev_prev), Some(prev)) => {
				let result = prev_prev + prev;
				self.prev_prev = self.prev;
				self.prev = Some(result);
				Some(result)
			},
			(None, Some(prev)) => {
				const SECOND_FIBONACCI_NUMBER: u32 = 1;
				self.prev_prev = Some(prev);
				self.prev = Some(SECOND_FIBONACCI_NUMBER);
				Some(SECOND_FIBONACCI_NUMBER)
			},
			(None, None) => {
				const FIRST_FIBONACCI_NUMBER: u32 = 0;
				self.prev = Some(FIRST_FIBONACCI_NUMBER);
				Some(FIRST_FIBONACCI_NUMBER)
			},
			_ => unreachable!(),
		}
	}
}
