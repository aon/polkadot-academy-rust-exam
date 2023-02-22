use pba_entrance_exam::b_multiple_choice::*;

pub fn sanity_check(f: &dyn Fn() -> char) {
	assert!("abcde".contains(f()), "{}", "You have not returned an answer a, b, c, d, or e.")
}

#[test]
fn answer_1_a_sanity_check() {
	sanity_check(&answer_1_a)
}

#[test]
fn answer_1_b_sanity_check() {
	sanity_check(&answer_1_b)
}

#[test]
fn answer_1_c_sanity_check() {
	sanity_check(&answer_1_c)
}

#[test]
fn answer_1_d_sanity_check() {
	sanity_check(&answer_1_d)
}

#[test]
fn answer_2_sanity_check() {
	sanity_check(&answer_2)
}

#[test]
fn answer_3_a_sanity_check() {
	sanity_check(&answer_3_a)
}

#[test]
fn answer_3_b_sanity_check() {
	sanity_check(&answer_3_b)
}

#[test]
fn answer_4_a_sanity_check() {
	sanity_check(&answer_4_a)
}

#[test]
fn answer_4_b_sanity_check() {
	sanity_check(&answer_4_b)
}

#[test]
fn answer_5_sanity_check() {
	sanity_check(&answer_5)
}

#[test]
fn answer_6_sanity_check() {
	sanity_check(&answer_6)
}

#[test]
fn answer_7_sanity_check() {
	sanity_check(&answer_7)
}
