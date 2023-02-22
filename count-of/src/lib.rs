// This problem is OPTIONAL

//use inflector::Inflector;
use quote::quote;

#[proc_macro_derive(CountOf)]
pub fn count_of(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _input = syn::parse_macro_input!(input as syn::ItemEnum);
    let output = quote!();
    proc_macro::TokenStream::from(output)
}
