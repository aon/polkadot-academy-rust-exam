use pba_entrance_exam::{k_macros::*, *};
use std::collections::HashMap;

#[derive(CountOf)]
pub enum Foo {
	Bar,
	Baz,
}

#[test]
fn map() {
	let macro_generated: HashMap<u32, u32> = map![1 => 2, 3 => 4, 5 => 6];
	let mut expected = HashMap::<u32, u32>::new();
	expected.insert(1, 2);
	expected.insert(3, 4);
	expected.insert(5, 6);

	assert_eq!(macro_generated, expected);
}

// #[test]
// fn impl_get() {
// 	impl_get!(
// 		// should generate `struct Foo` that has a function `get() -> u32`
// 		Foo: u32 = 10;
// 		// should generate `pub struct Foo` that has a function `get() -> u32`
// 		pub struct Bar: u32 = 42;
// 		// should generate `pub struct Foo` that has a constant function `get() -> u32`
// 		pub const Baz: u32 = 42;
// 	);

// 	assert_eq!(Foo::get(), 10);
// 	assert_eq!(Bar::get(), 42);
// 	const CONST: u32 = Baz::get();
// 	assert_eq!(CONST, 42);
// }

// #[test]
// fn impl_for_tuples() {
// 	<(Module1, Module2, Module3, Module4) as OnInitialize>::on_initialize()
// }

// #[test]
// fn count_of() {
// 	assert_eq!(vec![Foo::Bar, Foo::Bar, Foo::Baz, Foo::Bar].bar_count(), 3);
// 	assert_eq!(vec![Foo::Bar, Foo::Bar, Foo::Baz, Foo::Bar].baz_count(), 1);
// }
