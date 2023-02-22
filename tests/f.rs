use pba_entrance_exam::f_iterators::*;

#[test]
fn sum_of_squares_1() {
	let initial = [1u32, 2, 3].into_iter();

	assert_eq!(14, sum_of_squares(initial));
}

#[test]
fn bounded_absolute_values_1() {
	let initial = [1, 5, -5, 101, -200, 9, 0].into_iter();
	let expected = vec![1u32, 5, 5, 9, 0];

	assert_eq!(expected, bounded_absolute_values(initial).collect::<Vec<_>>());
}

#[test]
fn first_n_even_1() {
	let initial = [3u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();
	let expected = vec![2u32, 4, 6];

	assert_eq!(expected, first_n_even(initial).unwrap().collect::<Vec<_>>());
}

#[test]
fn first_n_even_2() {
	let initial = [3u32, 1, 3, 5, 7, 9].into_iter();

	assert_eq!(Vec::<u32>::new(), first_n_even(initial).unwrap().collect::<Vec<_>>());
}

#[test]
fn square_whole_numbers_1() {
	assert_eq!(vec![0, 1, 4, 9, 16, 25], square_whole_numbers().take(6).collect::<Vec<_>>());
}

#[test]
fn fibonacci_1() {
	let fib = Fibonacci::default();
	let expected = vec![0u32, 1, 1, 2, 3, 5, 8];

	assert_eq!(expected, fib.take(7).collect::<Vec<_>>());
}
