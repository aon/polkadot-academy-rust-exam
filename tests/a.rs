use pba_entrance_exam::a_honor_code::*;

pub fn has_honor(f: &dyn Fn() -> bool) {
	assert!(
		f(),
		"Thank you for your honesty in letting us know you have not followed the honor code."
	)
}

#[test]
fn honor_code_respected() {
	has_honor(&multiple_choice_closed_book);
	has_honor(&exam_done_independently);
	has_honor(&multiple_choice_no_run);
	has_honor(&coding_no_copy);
	has_honor(&coding_no_external_deps);
	has_honor(&coding_no_ai_helpers);
	has_honor(&exam_no_share);
}
