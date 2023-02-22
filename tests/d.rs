use pba_entrance_exam::d_pattern_matching::*;

#[test]
fn test_match_1_true() {
	let strs =
		vec!["Hello".to_string(), "World".to_string(), "PBA".to_string(), "PBASDFGR".to_string()];
	assert!(match_1(strs))
}

#[test]
fn test_match_1_false() {
	let strs = vec!["Hello".to_string(), "World".to_string(), "PBA".to_string()];
	assert!(!match_1(strs))
}

#[test]
fn test_match_2() {
	let strs = vec![
		"PBAHello".to_string(),
		"World".to_string(),
		"PBA".to_string(),
		"PBASDFGR".to_string(),
	];
	assert!(match_2(strs))
}

#[test]
fn test_match_3() {
	assert!(match_3((true, false, true)))
}

#[test]
fn test_match_4_true() {
	assert!(match_4(Ok(6)))
}

#[test]
fn test_match_4_false() {
	assert!(!match_4(Err("bogus")))
}
