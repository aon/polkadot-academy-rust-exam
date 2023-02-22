use pba_entrance_exam::h_advanced_traits::*;

trait ToBTU {
	fn to_btu(self) -> BTU;
}

impl<T: Into<BTU>> ToBTU for T {
	fn to_btu(self) -> BTU {
		self.into()
	}
}

#[test]
fn nuclear() {
	let nr = NuclearReactor;
	assert_eq!(nr.provide_energy(FuelContainer::new(10)).to_btu(), 9900);
	assert_eq!(nr.provide_energy(FuelContainer::new(10)).to_btu(), 9900);
}

#[test]
fn ic_1() {
	let ic = InternalCombustion::<3>::new(120);
	assert_eq!(ic.provide_energy(FuelContainer::new(10)).to_btu(), 1000);
	assert_eq!(ic.provide_energy(FuelContainer::new(10)).to_btu(), 1000);
	assert_eq!(ic.provide_energy(FuelContainer::new(10)).to_btu(), 1000);
	assert_eq!(ic.provide_energy(FuelContainer::new(10)).to_btu(), 990);
}

#[test]
fn omni_1() {
	let og = OmniGenerator::<100>;
	assert_eq!(og.provide_energy(FuelContainer::<Uranium>::new(10)).to_btu(), 10000);
	assert_eq!(og.provide_energy(FuelContainer::<Diesel>::new(10)).to_btu(), 1000);
	assert_eq!(og.provide_energy(FuelContainer::<LithiumBattery>::new(10)).to_btu(), 2000);
}

#[test]
fn mixed_1() {
	assert_eq!(Mixed::<Diesel, LithiumBattery>::energy_density().to_btu(), 150);
}

#[test]
fn custom_mixed_1() {
	// custom with 50 is the same as Mixed.
	assert_eq!(
		CustomMixed::<50, Diesel, LithiumBattery>::energy_density().to_btu(),
		Mixed::<Diesel, LithiumBattery>::energy_density()
	);
}
