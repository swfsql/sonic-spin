#![feature(proc_macro_hygiene)]
#![feature(external_doc)]

extern crate proc_macro;
extern crate proc_macro2;

mod resyn;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn sonic_spin(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as resyn::Expr);

    let reparsed = quote! {
       #input
    };

    let quoted = format!(" ==> <  {}  >\n", &reparsed);
    println!("{}", &quoted);

    reparsed.into()
}

