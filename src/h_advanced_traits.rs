// First, we are going to introduce some units of energy. For whatever reason, we prefer BTU above
// Joules and Calories, but we want to support all 3 of these in this module. Double check the
// conversion methods, and make sure you fully understand them.

use std::marker::PhantomData;

// You may uncomment and use the following import if you need it. You may also read its
// documentation at https://doc.rust-lang.org/std/cell/struct.RefCell
use std::cell::RefCell;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Joule(u32);
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Calorie(u32);

pub type BTU = u32;

impl From<Joule> for BTU {
	fn from(j: Joule) -> Self {
		j.0 / 1055
	}
}

impl From<BTU> for Joule {
	fn from(b: BTU) -> Self {
		Self(b * 1055)
	}
}

impl From<Calorie> for BTU {
	fn from(c: Calorie) -> Self {
		c.0 / 251
	}
}

impl From<BTU> for Calorie {
	fn from(b: BTU) -> Self {
		Calorie(b * 251)
	}
}

// Now, we start defining some types of fuel.

/// A technology for storing energy for later consumption.
pub trait Fuel {
	/// The output unit of the energy density.
	///
	/// Think about this: why did we chose this to be an associated type rather than a generic?
	type Output: Into<BTU> + From<BTU>;

	/// The amount of energy contained in a single unit of fuel.
	fn energy_density() -> Self::Output;
}

pub struct Diesel;
impl Fuel for Diesel {
	type Output = Joule;
	fn energy_density() -> Self::Output {
		Joule::from(100)
	}
}

pub struct LithiumBattery;
impl Fuel for LithiumBattery {
	type Output = Calorie;
	fn energy_density() -> Self::Output {
		Calorie::from(200)
	}
}

pub struct Uranium;
impl Fuel for Uranium {
	type Output = Joule;
	fn energy_density() -> Self::Output {
		Joule::from(1000)
	}
}

/// A container for any fuel type.
pub struct FuelContainer<F: Fuel> {
	/// the amount of fuel.
	amount: u32,
	/// Note: Why do we need this? Fuel doesn't really have any methods that require `&self` on it,
	/// so any information that we can get, we can get from `F` as **TYPE**, we don't really need
	/// to store an instance of `F`, like `fuel: F` as a struct field. But to satisfy the compiler,
	/// we muse use `F` somewhere. This is the perfect use case of `PhantomData`.
	_marker: PhantomData<F>,
}

impl<F: Fuel> FuelContainer<F> {
	pub fn new(amount: u32) -> Self {
		Self { amount, _marker: Default::default() }
	}
}

/// Something that can provide energy from a given `F` fuel type, like a power-plant.
pub trait ProvideEnergy<F: Fuel> {
	/// consume the fuel container and return the created energy, based on the power density of the
	/// fuel and potentially other factors.
	///
	/// Some fuel providers might have some kind of decay or inefficiency, which should be reflected
	/// here. Otherwise, [`provide_energy_with_efficiency`] or [`provide_energy_ideal`] might be
	/// good enough.
	///
	/// Not all `ProvideEnergy` implementations need to have internal state. Therefore, this
	/// interface accepts `&self`, not `&mut self`. You might need to use special language features
	/// to overcome this.
	fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output;

	/// Convert the amount of fuel in `f` with an exact efficiency of `e`.
	///
	/// NOTE: all efficiencies are interpreted as u8 values that can be at most 100, and represent a
	/// percent.
	///
	/// This method must be provided as it will be the same in all implementations.
	fn provide_energy_with_efficiency(&self, f: FuelContainer<F>, e: u8) -> <F as Fuel>::Output {
		let energy_density: BTU = <F as Fuel>::energy_density().into();
		let ideal_energy = energy_density * f.amount;
		<F as Fuel>::Output::from(ideal_energy * u32::from(e) / 100)
	}

	/// Same as above, but with an efficiency of 100.
	///
	/// This method must be provided as it will be the same in all implementations.
	fn provide_energy_ideal(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
		let energy_density: BTU = <F as Fuel>::energy_density().into();
		let ideal_energy = energy_density * f.amount;
		<F as Fuel>::Output::from(ideal_energy)
	}
}

/// A nuclear reactor that can only consume `Uranium` and provide energy with 99% efficiency.
pub struct NuclearReactor;
impl ProvideEnergy<Uranium> for NuclearReactor {
	fn provide_energy(&self, f: FuelContainer<Uranium>) -> <Uranium as Fuel>::Output {
		let e: u8 = 99;
		let energy_density: BTU = Uranium::energy_density().into();
		let ideal_energy = energy_density * f.amount;
		<Uranium as Fuel>::Output::from(ideal_energy * u32::from(e) / 100)
	}
}

/// A combustion engine that can only consume `Diesel`.
///
/// The `DECAY` const must be interpreted as such: per every `DECAY` times `provide_energy` is
/// called on an instance of this type, the efficiency should reduce by one. The initial efficiency
/// must be configurable with a `fn new(efficiency: u8) -> Self`.
pub struct InternalCombustion<const DECAY: u32> {
	called_times: RefCell<u32>,
	initial_efficiency: u8,
}

impl<const DECAY: u32> InternalCombustion<DECAY> {
	pub fn new(efficiency: u8) -> Self {
		let efficiency = if efficiency > 100 { 100 } else { efficiency };
		Self { called_times: RefCell::new(0), initial_efficiency: efficiency }
	}
}

impl<const DECAY: u32> ProvideEnergy<Diesel> for InternalCombustion<DECAY> {
	fn provide_energy(&self, f: FuelContainer<Diesel>) -> <Diesel as Fuel>::Output {
		// Calculate current efficiency
		let called_times = self.called_times.take();
		let efficiency_decay = called_times.checked_div(DECAY).unwrap_or(0);
		let efficiency = self
			.initial_efficiency
			.saturating_sub(efficiency_decay.try_into().unwrap_or(u8::MAX));
		self.called_times.replace(called_times.saturating_add(1));

		// Calculate energy
		let energy_density: BTU = Diesel::energy_density().into();
		let ideal_energy = energy_density * f.amount;
		<Diesel as Fuel>::Output::from(ideal_energy * u32::from(efficiency) / 100)
	}
}

/// A hypothetical device that can, unlike the `InternalCombustion`, consume **any fuel** that's of
/// type `trait Fuel`. It can provide a fixed efficiency regardless of fuel type. As before,
/// EFFICIENCY is a u8 whose value should not exceed 100 and is interpreted as a percent.
pub struct OmniGenerator<const EFFICIENCY: u8>;

impl<const EFFICIENCY: u8, F: Fuel> ProvideEnergy<F> for OmniGenerator<EFFICIENCY> {
	fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
		let efficiency = if EFFICIENCY > 100 { 100 } else { EFFICIENCY };
		let energy_density: BTU = <F as Fuel>::energy_density().into();
		let ideal_energy = energy_density * f.amount;
		<F as Fuel>::Output::from(ideal_energy * u32::from(efficiency) / 100)
	}
}

/// A type that can wrap two different fuel types and mix them together.
///
/// The energy density of the new fuel type is the average of the two given, once converted to BTU.
/// The output unit should also be BTU.
///
/// This can represent a new fuel type, thus it must implement `Fuel`.
pub struct Mixed<F1: Fuel, F2: Fuel>(PhantomData<(F1, F2)>);

impl<F1: Fuel, F2: Fuel> Fuel for Mixed<F1, F2> {
	type Output = BTU;

	fn energy_density() -> Self::Output {
		let f1_energy_density: BTU = F1::energy_density().into();
		let f2_energy_density: BTU = F2::energy_density().into();
		(f1_energy_density + f2_energy_density) / 2
	}
}

// Now think about how you can make the mixer configurable, such that it would produce a new fuel
// with an energy density that is more influences by one type than the other.
//
// For example, you have a mixer of F1, F2, and some coefficient C1, where the energy density of the
// mixture is `F1 * C1 + F2 * (1 - C1) )` where `C1` is a ratio (which you have to represent again
// with a u8 percent).
//
// The main trick is to overcome the fact that `fn energy_density` does not take in a `self`, so the
// coefficients need to be incorporated in some other way (you've already seen examples of that in
// this file ;)).
pub struct CustomMixed<const C: u8, F1, F2>(PhantomData<(F1, F2)>);

impl<const C: u8, F1: Fuel, F2: Fuel> Fuel for CustomMixed<C, F1, F2> {
	type Output = BTU;

	fn energy_density() -> Self::Output {
		let f1_energy_density: BTU = F1::energy_density().into();
		let f2_energy_density: BTU = F2::energy_density().into();
		f1_energy_density * u32::from(C) / 100 + f2_energy_density * (100 - u32::from(C)) / 100
	}
}

// Now, any of our existing energy providers can be used with a mix fuel.
/// A function that returns the energy produced by the `OmniGenerator` with efficiency of 80%, when
/// the fuel type is a mix of `Diesel` as `LithiumBattery`;
pub fn omni_80_energy(amount: u32) -> BTU {
	let omni_generator = OmniGenerator::<80> {};
	let fuel_container = FuelContainer::<Mixed<Diesel, LithiumBattery>>::new(amount);
	omni_generator.provide_energy(fuel_container)
}

// Finally, let's consider marker traits, and some trait bounds.

/// Some traits are just markers. They don't bring any additional functionality anything, other than
/// marking a type with some trait.
pub trait IsRenewable {}
impl IsRenewable for LithiumBattery {}

/// Define the following struct such that it only provides energy if the fuel is `IsRenewable`.
///
/// It has perfect efficiency.
pub struct GreenEngine<F: Fuel>(PhantomData<F>);

impl<F: Fuel + IsRenewable> ProvideEnergy<F> for GreenEngine<F> {
	fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
		let energy_density: BTU = <F as Fuel>::energy_density().into();
		let energy = energy_density * f.amount;
		<F as Fuel>::Output::from(energy)
	}
}

/// Define the following struct such that it only provides energy if the fuel's output type is
/// `BTU`.
///
/// It has perfect efficiency.
pub struct BritishEngine<F: Fuel>(PhantomData<F>);

impl<F: Fuel> ProvideEnergy<F> for BritishEngine<F>
where
	<F as Fuel>::Output: Into<BTU>,
{
	fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
		let energy_density: BTU = <F as Fuel>::energy_density().into();
		let energy = energy_density * f.amount;
		<F as Fuel>::Output::from(energy)
	}
}

// Congratulations! you have finished the advance trait section.
//
// Disclaimer: the types and traits that you are asked to implement in this module are by no means
// designed to be sensible. Instead, they are chosen to represent a typical, often difficult,
// pattern. Some are intentionally slightly convoluted to challenge you :). I am sure if we actually
// wanted to design a fuel system, we would do better.
