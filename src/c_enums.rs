/// A type representing distances in multiple units
#[derive(Clone, Copy)]
pub enum Distance {
	Meters(f32),
	Feet(f32),
	Miles(f32),
}

/// There are roughly 1609 meters in one mile
const METERS_PER_MILE: f32 = 1609.344;

/// There are roughly 3.3 feet in one meter
const FEET_PER_METER: f32 = 3.281;

impl Distance {
	/// Convert the given distance to meters
	pub fn to_meters(&self) -> Self {
		Distance::Meters(match *self {
			Self::Meters(d) => d,
			Self::Feet(d) => d / FEET_PER_METER,
			Self::Miles(d) => d * METERS_PER_MILE,
		})
	}

	/// Convert the given distance to feet
	pub fn to_feet(&self) -> Self {
		Distance::Feet(match *self {
			Self::Meters(d) => d * FEET_PER_METER,
			Self::Feet(d) => d,
			Self::Miles(d) => d * METERS_PER_MILE * FEET_PER_METER,
		})
	}

	/// Convert the given distance to miles
	pub fn to_miles(&self) -> Self {
		Distance::Miles(match *self {
			Self::Meters(d) => d / METERS_PER_MILE,
			Self::Feet(d) => d / FEET_PER_METER / METERS_PER_MILE,
			Self::Miles(d) => d,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn assert_equalish(converted: f32, expected: f32) {
		let delta = f32::abs(converted - expected);
		assert!(delta < 0.0001);
	}

	mod test_to_meters {
		use super::*;

		#[test]
		fn meters_to_meters() {
			let meters = 5.0;
			let expected_meters = meters;
			let converted_meters = Distance::Meters(meters).to_meters();

			match converted_meters {
				Distance::Meters(converted_meters) => {
					assert_equalish(converted_meters, expected_meters)
				},
				_ => unreachable!(),
			}
		}

		#[test]
		fn feet_to_meters() {
			let feet = 4.0;
			let expected_meters = 1.2192;
			let converted_meters = Distance::Feet(feet).to_meters();

			match converted_meters {
				Distance::Meters(converted_meters) => {
					assert_equalish(converted_meters, expected_meters)
				},
				_ => unreachable!(),
			}
		}

		#[test]
		fn miles_to_meters() {
			let miles = 12.0;
			let expected_meters = 19312.128;
			let converted_meters = Distance::Miles(miles).to_meters();

			match converted_meters {
				Distance::Meters(converted_meters) => {
					assert_equalish(converted_meters, expected_meters)
				},
				_ => unreachable!(),
			}
		}
	}

	mod test_to_feet {
		use super::*;

		#[test]
		fn meters_to_feet() {
			let meters = 5.0;
			let expected_feet = 16.405;
			let converted_feet = Distance::Meters(meters).to_feet();

			match converted_feet {
				Distance::Feet(converted_feet) => assert_equalish(converted_feet, expected_feet),
				_ => unreachable!(),
			}
		}

		#[test]
		fn feet_to_feet() {
			let feet = 4.0;
			let expected_feet = feet;
			let converted_feet = Distance::Feet(feet).to_feet();

			match converted_feet {
				Distance::Feet(converted_feet) => assert_equalish(converted_feet, expected_feet),
				_ => unreachable!(),
			}
		}

		#[test]
		fn miles_to_feet() {
			let miles = 12.0;
			let expected_feet = 63363.092;
			let converted_feet = Distance::Miles(miles).to_feet();

			match converted_feet {
				Distance::Feet(converted_feet) => assert_equalish(converted_feet, expected_feet),
				_ => unreachable!(),
			}
		}
	}

	mod test_to_miles {
		use super::*;

		#[test]
		fn meters_to_miles() {
			let meters = 5.0;
			let expected_miles = 0.00310686;
			let converted_miles = Distance::Meters(meters).to_miles();

			match converted_miles {
				Distance::Miles(converted_miles) => {
					assert_equalish(converted_miles, expected_miles)
				},
				_ => unreachable!(),
			}
		}

		#[test]
		fn feet_to_miles() {
			let feet = 4.0;
			let expected_miles = 0.000757576;
			let converted_miles = Distance::Feet(feet).to_miles();

			match converted_miles {
				Distance::Miles(converted_miles) => {
					assert_equalish(converted_miles, expected_miles)
				},
				_ => unreachable!(),
			}
		}

		#[test]
		fn miles_to_miles() {
			let miles = 12.0;
			let expected_miles = miles;
			let converted_miles = Distance::Miles(miles).to_miles();

			match converted_miles {
				Distance::Miles(converted_miles) => {
					assert_equalish(converted_miles, expected_miles)
				},
				_ => unreachable!(),
			}
		}
	}
}
