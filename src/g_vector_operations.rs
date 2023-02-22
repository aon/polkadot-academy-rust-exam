//! In the portion of the test, you will write a few functions that operate on Vectors.
//! The algorithms include sorts, searches, and reversals.
//! In addition to understanding the algorithm, this will test your understanding of Rust's
//! ownership model.

/// This is an in-place sort, so it moves data around in the original slice.
/// You may look up how the algorithm works (eg here https://en.wikipedia.org/wiki/Bubble_sort)
/// But you may not look up implementations of it in Rust.
pub fn bubble_sort(items: &mut [u32]) {
	for n in (1..items.len()).rev() {
		for i in 0..n {
			if items[i] > items[i + 1] {
				let temp = items[i];
				items[i] = items[i + 1];
				items[i + 1] = temp;
			}
		}
	}
}

/// This is a recursive sort, so you must use recursion.
/// This is NOT an in-place sort, so it will return a copy of the data in a new Vec.
/// You may look up how the algorithm works (eg here https://en.wikipedia.org/wiki/Merge_sort)
/// But you may not look up implementations of it in Rust.
pub fn merge_sort(items: &[u32]) -> Vec<u32> {
	if items.len() <= 1 {
		items.to_vec()
	} else {
		// Split items into two slices
		let middle = items.len() / 2;
		let left_items = merge_sort(&items[0..middle]);
		let right_items = merge_sort(&items[middle..]);

		// Perform merge
		let mut i = 0;
		let mut j = 0;
		let mut merged_items = Vec::new();

		while i < left_items.len() && j < right_items.len() {
			if left_items[i] < right_items[j] {
				merged_items.push(left_items[i]);
				i += 1;
			} else {
				merged_items.push(right_items[j]);
				j += 1;
			}
		}

		while i < left_items.len() {
			merged_items.push(left_items[i]);
			i += 1;
		}

		while j < right_items.len() {
			merged_items.push(right_items[j]);
			j += 1;
		}

		merged_items
	}
}

/// Reverse a slice in-place.
/// This is what the built-in `reverse` method does. You may NOT use that method here
/// https://doc.rust-lang.org/std/primitive.slice.html#method.reverse
pub fn reverse_in_place(items: &mut [u32]) {
	let mut i = 0;
	let mut j = items.len() - 1;

	while i < j {
		let temp = items[i];
		items[i] = items[j];
		items[j] = temp;
		i += 1;
		j -= 1;
	}
}

/// Create and return a Vec containing the same data as the parameter slice in reverse order.
pub fn reverse_copy(items: &[u32]) -> Vec<u32> {
	let mut reversed = Vec::new();

	for i in (0..items.len()).rev() {
		reversed.push(items[i]);
	}

	reversed
}

/// Search a slice for a particular item. Return the index of the first occurrence of that item.
/// You may look up how the algorithm works (eg here https://en.wikipedia.org/wiki/Linear_search)
/// But you may not look up implementations of it in Rust.
/// This is what the built-in `contains` method does. You may NOT use that method here.
/// https://doc.rust-lang.org/std/primitive.slice.html#method.contains
pub fn linear_search(items: &[u32], target: &u32) -> Option<usize> {
	for (i, item) in items.iter().enumerate() {
		if item == target {
			return Some(i);
		}
	}
	None
}

/// Search a slice for a particular item. Return the index of any occurrence of that item.
/// You may look up how the algorithm works (eg here https://en.wikipedia.org/wiki/Binary_search)
/// You may (and must) assume that the data is sorted.
/// But you may not look up implementations of it in Rust.
/// This is what the built-in `binary_search` method does. You may NOT use that method here.
/// https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
pub fn binary_search(items: &[u32], target: &u32) -> Option<usize> {
	let mut i = 0;
	let mut j = items.len() - 1;

	while i <= j {
		let middle = (i + j) / 2;

		if items[middle] < *target {
			i = middle + 1;
		} else if items[middle] > *target {
			j = middle - 1;
		} else {
			return Some(middle);
		}
	}

	None
}
