use pba_entrance_exam::g_vector_operations::*;

#[test]
fn bubble_sort_success() {
	let mut original = vec![1u32, 7, 5, 4, 6];
	let sorted = vec![1u32, 4, 5, 6, 7];

	bubble_sort(&mut original[..]);

	assert_eq!(original, sorted[..])
}

#[test]
fn merge_sort_success() {
	let original = vec![1u32, 7, 5, 4, 6];
	let sorted = vec![1u32, 4, 5, 6, 7];

	assert_eq!(merge_sort(&original[..]), sorted[..])
}

#[test]
fn reverse_in_place_success() {
	let mut original = vec![1u32, 7, 5, 4, 6];
	let reversed = vec![6u32, 4, 5, 7, 1];

	reverse_in_place(&mut original[..]);

	assert_eq!(original, reversed[..])
}

#[test]
fn reverse_copy_success() {
	let original = vec![1u32, 7, 5, 4, 6];
	let reversed = vec![6u32, 4, 5, 7, 1];

	assert_eq!(reverse_copy(&original[..]), reversed[..])
}

#[test]
fn linear_search_success() {
	let haystack = vec![1u32, 7, 5, 4, 6];
	let needle = 5u32;

	assert_eq!(linear_search(&haystack[..], &needle), Some(2))
}

#[test]
fn binary_search_success() {
	let haystack = vec![1u32, 4, 5, 6, 7];
	let needle = 5u32;

	assert_eq!(binary_search(&haystack[..], &needle), Some(2))
}
