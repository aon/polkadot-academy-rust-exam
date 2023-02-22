//! Here we test your ability to write macros.
//!
//! Make sure these macros are importable from the root of your
//! crate, and usable in an external crate.

// A common Rust macro is `vec![...]` used for creating vectors of literal values. Without this
// macro, one would need to create an empty vector, and push each item to it individually.
//
// Your task here is to create an analogous macro for creating hashmaps pre-populated with literal
// values. The macro should be called like follows:
//
// let map1: HashMap<u32, u32> = map![1 =>2, 3 => 4, 5 => 6)];
#[macro_export]
macro_rules! map {
	( $( $x:expr => $y:expr ),* ) => {
        {
            let mut temp_hash_map = HashMap::new();
            $(
                temp_hash_map.insert($x, $y);
            )*
            temp_hash_map
        }
    };
}

// OPTIONAL
// Next, write a macro that implements a `get` function on a type. See the test case below for the
// expected syntax.
//
// We want to same macro to be able to parse multiple items, and each item can be either `type Foo
// ..` or `const Foo ..`. The former should generate a `fn get()`, and the latter a `const fn
// get()`.
#[macro_export]
macro_rules! impl_get {
	(($todo:tt)) => {
		// This problem is OPTIONAL
		todo!("OPTIONAL")
	};
}

// Lastly, consider the following setup. A number of different types/structs all implement a common
// `trait OnInitialize`.
pub struct Module1;
pub struct Module2;
pub struct Module3;
pub struct Module4;

pub trait OnInitialize {
	fn on_initialize();
}

impl OnInitialize for Module1 {
	fn on_initialize() {}
}
impl OnInitialize for Module2 {
	fn on_initialize() {}
}
impl OnInitialize for Module3 {
	fn on_initialize() {}
}
impl OnInitialize for Module4 {
	fn on_initialize() {}
}

// OPTIONAL: Now, we want to write a macro that reduces the boilerplate of implementing
// `OnInitialize` on tuples of elements that each individually implement `OnInitialize`. Assume we
// want to support tuples of up to 12 elements.
//
// For example, calling `impl_for_tuples!()` should expand to the following code, plus several more
// impl blocks supporting tuples of up to 12 elements.
//
// ```rust
// impl<T1: OnInitialize> OnInitialize for (T1,) {
//    fn on_initialize() {
//        T1::on_initialize();
//    }
// }
//
// impl<T1: OnInitialize, T2: OnInitialize> OnInitialize for (T1, T2) {
//    fn on_initialize() {
//        T1::on_initialize();
//        T2::on_initialize();
//    }
// }
//
// // And several more impl blocks supporting up to 12 elements
// ```
#[macro_export]
macro_rules! impl_for_tuples {
	(($todo:tt)) => {
		// This problem is OPTIONAL
		todo!("OPTIONAL")
	};
}

// OPTIONAL
// The three problems in this file have all explored declarative macros. Rust also supports
// procedural macros. In the section on extension traits, we discussed a hypothetical
// derive macro (a type of procedural macro) that could count the occurrences of a particular
// enum variant in a collection. The template for such a macro is provided in the separate
// `count-of` crate. The macro is currently doing nothing. Complete it!

/// Another outcome macro similar to the one in the extension traits section.
/// The only difference is that this one derives the CountOf macro.
///
/// You may use this one to test your CountOf macro.
#[derive(count_of::CountOf, Clone)]
pub enum OtherOutcome {
	Ok,
	SomethingWentWrong,
	IDontKnow,
}
