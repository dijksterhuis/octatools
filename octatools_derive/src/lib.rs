//! Derive macros for boilerplate trait implementation in the `serde_octatrack`
//! library crate

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

/// Macro to create derivable trait for the standard implementation of
/// `serde_octatrack::DefaultsArray` on a type (i.e. an array with inferred
/// length based on type hints)
#[proc_macro_derive(DefaultsAsArray)]
pub fn defaults_as_array_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    let expanded = quote! {
        impl crate::DefaultsArray for #name {}
    };
    TokenStream::from(expanded)
}

/// Macro to create derivable trait for the standard implementation of
/// `serde_octatrack::DefaultsBoxedArray` on a type (i.e. a Boxed
/// serde-big-array Array with inferred length based on type hints)
#[proc_macro_derive(DefaultsAsBoxedBigArray)]
pub fn defaults_as_boxed_array_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    let expanded = quote! {
        impl DefaultsArrayBoxed for #name {}
    };
    TokenStream::from(expanded)
}
