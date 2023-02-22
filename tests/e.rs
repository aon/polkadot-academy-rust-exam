use pba_entrance_exam::e_common_traits::*;

#[test]
fn employee_from_string_success() {
	let s = String::from("Billy, 4, 5, 345");
	let billy = Employee { name: String::from("Billy"), experience: 4, wage: 5, uid: 345 };

	assert_eq!(billy, s.try_into().unwrap())
}

#[test]
fn employee_to_string_success() {
	let billy = Employee { name: String::from("Billy"), experience: 4, wage: 5, uid: 345 };

	assert_eq!(String::from("Billy, 4, 5, 345"), String::from(billy))
}

#[test]
fn employee_ord() {
	let billy = Employee { name: String::from("Billy"), experience: 4, wage: 5, uid: 345 };
	let susie = Employee { name: String::from("Susie"), experience: 5, wage: 5, uid: 347 };

	assert!(susie > billy);
}

#[test]
fn employee_neq() {
	let billy = Employee { name: String::from("Billy"), experience: 4, wage: 5, uid: 345 };
	let susie = Employee { name: String::from("Susie"), experience: 5, wage: 5, uid: 347 };

	assert!(susie != billy);
}
