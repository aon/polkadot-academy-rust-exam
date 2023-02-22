use pba_entrance_exam::i_extension_traits::*;

#[test]
fn simple_functions() {
	let x = vec![Outcome::Ok, Outcome::Ok, Outcome::IDontKnow];

	assert_eq!(ok_count(x.clone()), 2);
	assert_eq!(i_dont_know_count(x.clone()), 1);
	assert_eq!(something_went_wrong_count(x), 0);
}

#[test]
fn extension_trait() {
	let x = vec![Outcome::Ok, Outcome::Ok, Outcome::IDontKnow];

	assert_eq!(x.ok_count(), 2);
	assert_eq!(x.i_dont_know_count(), 1);
	assert_eq!(x.something_went_wrong_count(), 0);
}
