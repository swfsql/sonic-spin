#![feature(proc_macro_hygiene)]
#![feature(external_doc)]
#![doc(include = "../README.md")]

extern crate proc_macro;
extern crate proc_macro2;

mod resyn;
use proc_macro::TokenStream;
use quote::quote;

/// Changes the `Block` parsing syntax so that the `::()` postfix
/// serves as a general postfix operator.
#[proc_macro]
pub fn sonic_spin(item: TokenStream) -> TokenStream {
    let rebraced = {
        use std::str::FromStr;
        let rebraced: String = String::from("{") + &item.to_string() + &"}";
        TokenStream::from_str(&rebraced).unwrap()
    };

    let input = syn::parse_macro_input!(rebraced as resyn::expr::Block);
    let reparsed = quote! {
       #input
    };

    // let quoted = format!(" ==> <  {}  >\n", &reparsed);
    // println!("{}", &quoted);

    reparsed.into()
}
