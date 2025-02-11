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
/// `serde_octatrack::Encode` on a type (i.e. calling `bincode::serialise`).
///
/// Is also used on types like YAML config files which need to be passed to functions that require
/// the `Encode` trait
#[proc_macro_derive(Encodeable)]
pub fn encode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    let expanded = quote! {
        impl crate::Encode for #name {}
    };
    TokenStream::from(expanded)
}

/// Macro to create derivable trait for the standard implementation of
/// `serde_octatrack::Decode` on a type (i.e. calling `bincode::deserialise`)
///
/// Is also used on types like YAML config files which need to be passed to
/// functions that require the `Decode` trait.
///
/// See octatools-bin/actions/banks/yaml.rs for an example.
#[proc_macro_derive(Decodeable)]
pub fn decode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    let expanded = quote! {
        impl crate::Decode for #name {}
    };
    TokenStream::from(expanded)
}

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
