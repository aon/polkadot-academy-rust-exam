use pba_entrance_exam::c_enums::*;

#[test]
fn feet_to_miles() {
	let feet = Distance::Feet(4.1);
	let miles = feet.to_miles();

	let expected_miles = 0.000776f32;

	match miles {
		Distance::Miles(converted_miles) => {
			let delta = f32::abs(converted_miles - expected_miles);
			assert!(delta < 0.00001);
		},
		_ => assert!(false),
	}
}
