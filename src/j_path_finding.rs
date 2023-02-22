//! In this portion, you will write a path finding algorithm that finds a path through a network of
//! hiking trails that connect several campsites.
//!
//! Some trails are inherently one way. For example they may involve sliding down
//! a zipline, or diving off a cliff into a lake, which cannot be traveled in the opposite
//! direction. Other trails are normal hiking trails and can be hiked in either direction, but even
//! these trails may have different difficulties depending on the direction of travel. For example
//! they may have a steep hill and hiking downhill is easier than hiking uphill. For these reasons,
//! ALL TRAILS ARE MODELED AS ONE-WAY. If a hiking trail can be traveled in either direction, we
//! model it as two separate one-way trails.
//!
//! Your algorithm should be general enough that it can optimize the route for several different
//! desirable properties. For example, you should be able to find:
//!
//! * Shortest Distance
//! * Shortest travel time
//! * Least danger
//!
//! You may implement any path-finding algorithm you see fit for this portion, but if you don't
//! already know of one you like, we recommend studying and implementing [Dijkastra's Algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm).
//! > REMINDER: you may _not_ search for an algorithm in Rust for reference or to depend on here,
//! > per the honor code!

// NOTE: you may use any data structure you like from `std::collections`
use std::collections::{HashMap, HashSet};

/// Various types of terrain that may be encountered while traversing the trail network.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Terrain {
	PavedTrail,
	UnpavedTrail,
	RockyTrail,
	Water,
	Zipline,
	RopeBridge,
}

impl TryFrom<String> for Terrain {
	type Error = ();

	/// OPTIONAL
	fn try_from(s: String) -> Result<Terrain, ()> {
		match s.as_str() {
			"PavedTrail" => Ok(Terrain::PavedTrail),
			"UnpavedTrail" => Ok(Terrain::UnpavedTrail),
			"RockyTrail" => Ok(Terrain::RockyTrail),
			"Water" => Ok(Terrain::Water),
			"Zipline" => Ok(Terrain::Zipline),
			"RopeBridge" => Ok(Terrain::RopeBridge),
			_ => Err(()),
		}
	}
}

/// A level of skill that a hiker may attain in various forms of hiking.
#[derive(Debug, PartialEq, Eq)]
pub enum Skill {
	Beginner,
	Intermediate,
	Expert,
}

impl TryFrom<String> for Skill {
	type Error = ();

	/// OPTIONAL
	fn try_from(s: String) -> Result<Skill, ()> {
		// String encodings of skill variants are just their names.
		// This problem is OPTIONAL
		match s.as_str() {
			"Beginner" => Ok(Skill::Beginner),
			"Intermediate" => Ok(Skill::Intermediate),
			"Expert" => Ok(Skill::Expert),
			_ => Err(()),
		}
	}
}

/// A one-way trail that can be traveled from the starting campsite to the ending campsite.
/// Every campsite in the network has a unique name.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Trail {
	/// The name of the campsite where this trail begins
	pub start: String,
	/// The name of the trail where this campsite ends
	pub end: String,
	/// The distance traveled along the trail in meters
	pub distance: u32,
	/// The type of terrain that must be traversed in this trail
	pub terrain: Terrain,
	/// The danger encountered along the way. A Danger rating is a number between 0 and 255.
	/// More dangerous paths have a higher danger rating. For example, a flat paved trail has a
	/// danger rating of 0. A trail that involves diving off a cliff, or passing through mountain
	/// lion territory has a danger rating over 200.
	pub danger: u8,
}

// Parse trail data from a string.
// It is recommended to use your implementation for Terrain as a helper here.
impl TryFrom<String> for Trail {
	type Error = ();

	/// OPTIONAL
	fn try_from(s: String) -> Result<Trail, ()> {
		// The encoding of trail information is as follows:
		//
		// Starting Site => Ending Site: Distance (Terrain) [Danger]
		//
		// ### Examples:
		// Mountain Top => Green Lake: 2000 (PavedTrail) [19]
		// The Bird Watch => Lost Colony: 400 (Zipline) [20]
		let start = s.split("=>").next().ok_or(())?.trim().to_string();
		let end = s.split("=>").nth(1).ok_or(())?.split(":").next().ok_or(())?.trim().to_string();
		let distance: u32 = s
			.split(":")
			.nth(1)
			.ok_or(())?
			.split("(")
			.next()
			.ok_or(())?
			.trim()
			.parse()
			.map_err(|_| ())?;
		let terrain = Terrain::try_from(
			s.split("(").nth(1).ok_or(())?.split(")").next().ok_or(())?.trim().to_string(),
		)?;
		let danger: u8 = s
			.split("[")
			.nth(1)
			.ok_or(())?
			.split("]")
			.next()
			.ok_or(())?
			.trim()
			.parse()
			.map_err(|_| ())?;

		Ok(Trail { start, end, distance, terrain, danger })
	}
}

/// A hiker that is interested in traversing the trail network.
/// Hikers' skills are rated in multiple dimensions.
///
/// These ratings affect how fast a hiker can traverse various terrain, and in some cases, whether
/// they can traverse it at all.
#[derive(Debug, PartialEq, Eq)]
pub struct Hiker {
	/// The hikers ability to traverse trails on foot.
	pub hiking: Skill,
	/// The hikers ability to traverse trails over water.
	pub swimming: Skill,
	/// Whether the hiker is strong enough to hold their own body weight.
	pub strong: bool,
	/// Whether the hiker is brave enough to traverse scary, dangerous, or high elements.
	pub brave: bool,
}

// The default hiker is used in several of the functions below.
// Their name is Bill.
impl Default for Hiker {
	fn default() -> Self {
		Self { strong: true, brave: false, hiking: Skill::Intermediate, swimming: Skill::Expert }
	}
}

/// OPTIONAL
impl TryFrom<String> for Hiker {
	type Error = ();

	/// The format for a hiker is a simple comma separated list of keys and values.
	/// The keys MUST be in the right order or the string is invalid.
	///
	/// Example:
	/// "hiking: Beginner, swimming: Intermediate, strong: false, brave: false"
	fn try_from(value: String) -> Result<Self, ()> {
		fn get_value_after_colon(s: &str) -> Result<String, ()> {
			Ok(s.split(":").nth(1).ok_or(())?.trim().to_string())
		}

		let mut values = value.split(",");
		let hiking = Skill::try_from(get_value_after_colon(values.next().ok_or(())?)?)?;
		let swimming = Skill::try_from(get_value_after_colon(values.next().ok_or(())?)?)?;
		let strong = get_value_after_colon(values.next().ok_or(())?)?.parse().map_err(|_| ())?;
		let brave = get_value_after_colon(values.next().ok_or(())?)?.parse().map_err(|_| ())?;

		Ok(Hiker { hiking, swimming, strong, brave })
	}
}

impl Hiker {
	/// Returns the time it takes a hiker to traverse a trail, if they can traverse it at all.
	/// If the hiker cannot traverse the terrain, return None.
	///
	/// ### Rope  Bridges:
	/// Hikers can only traverse a rope bridge if they are brave. When traversing a rope bridge,
	/// the travel time is equal to the length of the bridge.
	///
	/// ### Zip Lines:
	/// Hikers can only traverse ziplines if they are both brave and strong. When traversing a
	/// zipline, the travel time is 1/10 of the length of the zipline (use integer division).
	///
	/// ### Water:
	/// All hikers can swim, but not at the same level.
	/// - Beginner swimmer travel time is 9X the length of the swim.
	/// - Intermediate swimmer travel time is 6X the length of the swim.
	/// - Expert swimmer travel time is 3X the length of the swim.
	///
	/// ### Hiking:
	/// All hikers can hike! But the travel time depends on both the terrain and the hikers ability.
	/// Use this table to find a hiker's speed multiplier. For multipliers less than 1, use integer
	/// division.
	///           Advanced | Intermediate | Beginner |
	///         | -------- | ------------ | -------- |
	///   Rocky |    1X    |      2X      |    4X    |
	/// Unpaved |   1/2X   |      1X      |    2X    |
	///   Paved |   1/4X   |     1/2X     |    1X    |
	pub fn travel_time(&self, terrain: &Terrain, distance: u32) -> Option<u32> {
		match terrain {
			Terrain::RopeBridge if self.brave => Some(distance),
			Terrain::Zipline if self.brave && self.strong => Some(distance / 10),
			Terrain::Water => {
				let multiplier = match self.swimming {
					Skill::Beginner => 9,
					Skill::Intermediate => 6,
					Skill::Expert => 3,
				};
				Some(multiplier * distance)
			},
			Terrain::RockyTrail => match self.hiking {
				Skill::Beginner => Some(4 * distance),
				Skill::Intermediate => Some(2 * distance),
				Skill::Expert => Some(distance),
			},
			Terrain::UnpavedTrail => match self.hiking {
				Skill::Beginner => Some(2 * distance),
				Skill::Intermediate => Some(distance),
				Skill::Expert => Some(distance / 2),
			},
			Terrain::PavedTrail => match self.hiking {
				Skill::Beginner => Some(distance),
				Skill::Intermediate => Some(distance / 2),
				Skill::Expert => Some(distance / 4),
			},
			_ => None,
		}
	}
}

/// OPTIONAL
/// This is the main path-finding function. It should be abstract enough that it can solve all of
/// the more specific path finding problems in this module. That is, all of the following problem
/// specific functions, should call this function to do the heavy lifting.
///
/// Given a hiker, their starting and ending points, a trail network, and a function that determines
/// the cost of the hiker traversing a given trail, determine whether the hiker can reach the
/// destination at all, and if they can, the minimal total cost reaching the destination.
pub fn optimal_path(
	start: String,
	destination: String,
	hiker: &Hiker,
	trails: impl Iterator<Item = Trail>,
	cost_function: impl Fn(&Hiker, &Trail) -> Option<u32>,
) -> Option<u32> {
	// Collect trails to be reusable (could be improved)
	let trails = Vec::from_iter(trails);

	// Create set of unvisited nodes
	let mut unvisited = HashSet::new();
	for trail in &trails {
		unvisited.insert(trail.start.clone());
		unvisited.insert(trail.end.clone());
	}

	// Create a map of node distances
	let mut distances = HashMap::new();
	for node in &unvisited {
		distances.insert(node.clone(), u32::MAX);
	}

	// Set start node distance to 0
	distances.insert(start.clone(), 0);

	// Evaluate smaller distance per node
	let mut current_node = start.clone();

	loop {
		// Set node as visited
		unvisited.remove(&current_node);

		// Get all trails that start at current node
		let current_trails = trails.iter().filter(|trail| trail.start == current_node);

		// Update distances for all nodes that can be reached from current node
		for trail in current_trails {
			let distance = cost_function(hiker, trail);
			if distance.is_none() {
				continue;
			}
			let distance = distance.unwrap();

			let new_distance = distances[&current_node] + distance;
			if new_distance < distances[&trail.end] {
				distances.insert(trail.end.clone(), new_distance);
			}
		}

		// Check if destination is reached
		if current_node == destination {
			return Some(distances[&destination]);
		}

		// Otherwise find next node to visit
		let min_distance_node = distances
			.iter()
			.filter(|(node, _)| unvisited.contains(*node))
			.min_by_key(|(_, distance)| *distance);

		if min_distance_node.is_none() {
			return None;
		}

		current_node = min_distance_node.unwrap().0.clone();
	}
}

/// A specific path optimization problem. Bill wants to find the shortest (least distance) path from
/// "Green Lake" to "Prairie Meadows" though the given trail network.
///
/// Reminder: Bill is the Default::default() hiker.
pub fn bills_shortest_path_from_green_lake_to_prairie_meadows(
	trails: impl Iterator<Item = Trail>,
) -> Option<u32> {
	// This function is REQUIRED.
	// Although implementing the `optimal_path` helper is optional, this function, which calls
	// `optimal_path` is required. It will be graded to function against a reference implementation
	// and also manually for style.
	let hiker = Hiker::default();
	optimal_path(
		String::from("Green Lake"),
		String::from("Prairie Meadows"),
		&hiker,
		trails,
		|hiker, trail| Some(trail.distance),
	)
}

/// A specific path optimization problem. Bill wants to find the safest (least dangerous) path
/// between two arbitrary campsites.
///
/// Reminder: Bill is the Default::default() hiker.
pub fn bills_safest_path(
	start: String,
	destination: String,
	trails: impl Iterator<Item = Trail>,
) -> Option<u32> {
	// This function is REQUIRED.
	// Although implementing the `optimal_path` helper is optional, this function, which calls
	// `optimal_path` is required. It will be graded to function against a reference implementation
	// and also manually for style.
	let hiker = Hiker::default();
	optimal_path(
		String::from("Green Lake"),
		String::from("Prairie Meadows"),
		&hiker,
		trails,
		|hiker, trail| Some(trail.danger as u32),
	)
}

/// A specific path optimization problem. Hikers often want to find the fastest (least travel time)
/// path from "Green Lake" to "Prairie Meadows" though the given trail network.
pub fn fastest_path_from_green_lake_to_prairie_meadows(
	hiker: &Hiker,
	trails: impl Iterator<Item = Trail>,
) -> Option<u32> {
	// This function is REQUIRED.
	// Although implementing the `optimal_path` helper is optional, this function, which calls
	// `optimal_path` is required. It will be graded to function against a reference implementation
	// and also manually for style.
	let hiker = Hiker::default();
	optimal_path(
		String::from("Green Lake"),
		String::from("Prairie Meadows"),
		&hiker,
		trails,
		|hiker, trail| hiker.travel_time(&trail.terrain, trail.distance),
	)
}
