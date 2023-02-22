//! This portion will test your familiarity with some of Rust's common traits
//! and your ability to implement those traits in interesting ways
//! You need to remove all of the `todo!()`s. Most of them will need to be replaced
//! by some code, but some may be able to simply be deleted.

// NOTE: You will need to `use` something from the standard library to implement `Ord` and
// `PartialOrd` here.

use std::cmp::Ordering;

/// A record of an employee at a particular company
#[derive(Debug)]
pub struct Employee {
	/// The name the person likes to be called. Doesn't have to be their "passport name"
	pub name: String,
	/// Amount of experience (in months) the person has working at this company
	pub experience: u32,
	/// Hourly wage paid to this employee
	pub wage: u32,
	/// Unique identifier for this employee
	pub uid: u32,
}

// We want to consider two employee instances equal iff they have the same `uid`.

impl PartialEq for Employee {
	fn eq(&self, other: &Self) -> bool {
		self.uid == other.uid
	}
}

impl Eq for Employee {}

// We want to sort employees. First and foremost, employees are equal if they have the same
// `uid`, as explained above. For employees who are not equal, we sort by the value they
// bring to the company. Value is defined as the quotient of the experience they've acquired
// at the company divided by their wage. Use integer division for the purpose of this calculation.

impl PartialOrd for Employee {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let value = self.experience / self.wage;
		let other_value = other.experience / other.wage;
		Some(value.cmp(&other_value))
	}
}

impl Ord for Employee {
	fn cmp(&self, other: &Self) -> Ordering {
		let value = self.experience / self.wage;
		let other_value = other.experience / other.wage;
		value.cmp(&other_value)
	}
}

// We want to parse employee information from a string data
// The string data should be comma separated. Here are a few examples:
//
// * "Billy, 4, 5, 345" - Billy has been working here for 4 months and earns 5 token per hour. She
//   is employee #345
// * "Jose, 12, 6, 1" - Jose has been working here for 1 year (12 months) and earns 6
// tokens per hour. He is employee #1
//
// Any strings with the wrong number of commas or numbers too big for `u32` should return `Err(())`

impl TryFrom<String> for Employee {
	type Error = ();

	fn try_from(value: String) -> Result<Self, Self::Error> {
		let mut values_iterator = value.split(",");

		// Extract values from iterator
		let name = values_iterator.next().ok_or(())?;
		let experience = values_iterator.next().ok_or(())?;
		let wage = values_iterator.next().ok_or(())?;
		let uid = values_iterator.next().ok_or(())?;

		// Validate no more elements
		if matches!(values_iterator.next(), Some(_)) {
			return Err(());
		}

		// Parse values
		let name = name.trim().to_string();
		let experience: u32 = experience.trim().parse().map_err(|_| ())?;
		let wage: u32 = wage.trim().parse().map_err(|_| ())?;
		let uid: u32 = uid.trim().parse().map_err(|_| ())?;

		// Return struct
		Ok(Self { name, experience, wage, uid })
	}
}

// We also want to convert employees back into strings in the same format

impl From<Employee> for String {
	fn from(employee: Employee) -> Self {
		format!("{}, {}, {}, {}", employee.name, employee.experience, employee.wage, employee.uid)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_string_less_elements_error() {
		let s = String::from("Billy, 4, 5");
		assert_eq!(Err(()), Employee::try_from(s));
	}

	#[test]
	fn test_from_string_more_elements_error() {
		let s = String::from("Billy, 4, 5, 7, 8");
		assert_eq!(Err(()), Employee::try_from(s));
	}

	#[test]
	fn test_from_string_invalid_nums_error() {
		let s = String::from("Billy, 4a, 5, 7, 8");
		assert_eq!(Err(()), Employee::try_from(s));
	}

	#[test]
	fn test_from_string_num_overflow_error() {
		const MAX_U32_PLUS_ONE: u64 = u32::MAX as u64 + 1;
		let s = format!("Billy, {}, 5, 7", MAX_U32_PLUS_ONE);
		assert_eq!(Err(()), Employee::try_from(s));
	}

	#[test]
	fn test_eq() {
		let billy = Employee { name: String::from("Billy"), experience: 4, wage: 5, uid: 347 };
		let susie = Employee { name: String::from("Susie"), experience: 5, wage: 5, uid: 347 };

		assert_eq!(susie, billy);
	}
}
