use pba_entrance_exam::j_path_finding::*;

#[test]
fn terrain_from_string_1() {
	assert_eq!(Terrain::try_from("UnpavedTrail".to_string()), Ok(Terrain::UnpavedTrail))
}

#[test]
fn terrain_from_string_2() {
	// Notice to incorrect casing (capital P)
	assert_eq!(Terrain::try_from("UnPavedTrail".to_string()), Err(()))
}

#[test]
fn skill_from_string_1() {
	assert_eq!(Skill::try_from("Beginner".to_string()), Ok(Skill::Beginner))
}

#[test]
fn skill_from_string_2() {
	assert_eq!(Skill::try_from("Bogus".to_string()), Err(()))
}

#[test]
fn trail_from_string() {
	assert_eq!(
		Trail::try_from("Mountain Top => Green Lake: 2000 (PavedTrail) [19]".to_string()),
		Ok(Trail {
			start: "Mountain Top".into(),
			end: "Green Lake".into(),
			distance: 2000,
			terrain: Terrain::PavedTrail,
			danger: 19,
		})
	)
}

#[test]
fn hiker_from_string() {
	assert_eq!(
		Hiker::try_from(
			"hiking: Beginner, swimming: Intermediate, strong: false, brave: false".to_string()
		),
		Ok(Hiker {
			hiking: Skill::Beginner,
			swimming: Skill::Intermediate,
			strong: false,
			brave: false
		})
	)
}

#[test]
fn travel_time_1() {
	let hiker = Hiker::default();
	let tt = hiker.travel_time(&Terrain::RopeBridge, 5);

	assert_eq!(None, tt);
}

#[test]
fn travel_time_2() {
	let hiker = Hiker::default();
	let tt = hiker.travel_time(&Terrain::Zipline, 5);

	assert_eq!(None, tt);
}

#[test]
fn travel_time_3() {
	let hiker = Hiker::default();
	let tt = hiker.travel_time(&Terrain::UnpavedTrail, 5);

	assert_eq!(Some(5), tt);
}

#[test]
fn travel_time_4() {
	let hiker = Hiker::default();
	let tt = hiker.travel_time(&Terrain::Water, 5);

	assert_eq!(Some(15), tt);
}

#[test]
fn travel_time_5() {
	let hiker =
		Hiker { hiking: Skill::Expert, swimming: Skill::Expert, strong: false, brave: false };
	let tt = hiker.travel_time(&Terrain::PavedTrail, 5);

	assert_eq!(Some(1), tt);
}

fn test_path() -> impl Iterator<Item = Trail> {
	//                               /---------- A -------\
	//          1000 (RockyTrail) [60]                    400 (Zipline) [220]
	//           /                                                          \
	// Green Lake                                                            Prairie Meadow
	//           \                                                           /
	//            --- 1000 (PavedTrail) [40]           700 (UnpavedTrail) [30]
	//                                      \-- B -----/
	vec![
		Trail {
			start: "Green Lake".into(),
			end: "A".into(),
			distance: 1000,
			terrain: Terrain::RockyTrail,
			danger: 60,
		},
		Trail {
			start: "A".into(),
			end: "Prairie Meadow".into(),
			distance: 400,
			terrain: Terrain::Zipline,
			danger: 220,
		},
		Trail {
			start: "Green Lake".into(),
			end: "B".into(),
			distance: 1000,
			terrain: Terrain::PavedTrail,
			danger: 40,
		},
		Trail {
			start: "B".into(),
			end: "Prairie Meadow".into(),
			distance: 700,
			terrain: Terrain::UnpavedTrail,
			danger: 30,
		},
	]
	.into_iter()
}

#[test]
fn optimal_path_1() {
	let op = optimal_path(
		"Green Lake".into(),
		"Prairie Meadow".into(),
		&Default::default(),
		test_path(),
		// Find the shortest path overall, regardless of whether any particular hiker can traverse
		// it.
		|_, t| Some(t.distance),
	);

	assert_eq!(op, Some(1400), "OPTIONAL");
}

#[test]
fn optimal_path_2() {
	let op = optimal_path(
		"Green Lake".into(),
		"Prairie Meadow".into(),
		&Default::default(),
		test_path(),
		// Find the safest path overall, regardless of whether any particular hiker can traverse
		// it.
		|_, t| Some(t.danger as u32),
	);

	assert_eq!(op, Some(70), "OPTIONAL");
}
