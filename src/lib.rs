// NOTE: We allow dead code and unused variables and macros throughout the exam because they appear
// in many places in the starter code. You should remove this when you think you are done so that
// the CI can help you find potential mistakes.
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_macros)]

// NOTE: As the README outlines, your exam will be graded with some unit tests that will be
// executed as a [integration tests](https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html)
// included in this repository.
//
// This has some important implications:
//
// 1. Do not make anything private.
// 2. Do not change the name/signature of any function.
// 3. be aware that your macros are going to be executed in different file!
//
// The tests in each module `module_name` will import:
// `use pba_entrance_exam::*;`
// `use pba_entrance_exam::module_name::*;`

pub use count_of::CountOf;

pub mod a_honor_code;
pub mod b_multiple_choice;
pub mod c_enums;
pub mod d_pattern_matching;
pub mod e_common_traits;
pub mod f_iterators;
pub mod g_vector_operations;
pub mod h_advanced_traits;
pub mod i_extension_traits;
pub mod j_path_finding;
pub mod k_macros;
