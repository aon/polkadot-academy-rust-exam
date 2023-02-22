//! Complete the following functions using the pattern matching syntax. That includes the `match`
//! statement of the `matches!()` marco, if you feel like have a 1-liner.
//!
//! You can try and write them imperatively at first as well, but at the end of the day, we highly
//! prefer you to write them using the `match` or `matches!`.

/// Returns true if the last two strings in the vector start with `PBA`.
pub fn match_1(input: Vec<String>) -> bool {
	match &input[..] {
		[.., second_last, last] => last.starts_with("PBA") && second_last.starts_with("PBA"),
		_ => false,
	}
}

/// Returns true if the first and last string in the vector start with `PBA`.
pub fn match_2(input: Vec<String>) -> bool {
	match &input[..] {
		[first, .., last] => first.starts_with("PBA") && last.starts_with("PBA"),
		_ => false,
	}
}

/// Returns true if the first item in `input` is true.
pub fn match_3(input: (bool, bool, bool)) -> bool {
	match input {
		(true, ..) => true,
		_ => false,
	}
	// In this example I would've just returned input.0,
	// but pattern matching usage was required.
}

/// Returns true if the input is `Ok(x)` of some even `x`.
pub fn match_4(input: Result<u32, &'static str>) -> bool {
	matches!(input, Ok(x) if x % 2 == 0)
}

// Add some more tests for edge cases not covered in integration tests
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_match_1_short_vec() {
		let strs = vec!["PBAHello".to_string()];
		assert!(!match_1(strs));
	}

	#[test]
	fn test_match_1_empty_vec() {
		let strs = vec![];
		assert!(!match_1(strs));
	}

	#[test]
	fn test_match_2_short_vec() {
		let strs = vec!["PBAHello".to_string()];
		assert!(!match_2(strs));
	}

	#[test]
	fn test_match_2_empty_vec() {
		let strs = vec![];
		assert!(!match_2(strs));
	}

	#[test]
	fn test_match_3_false() {
		assert!(!match_3((false, true, true)))
	}

	#[test]
	fn test_match_4_odd() {
		assert!(!match_4(Ok(7)))
	}
}
