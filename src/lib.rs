#![feature(proc_macro_hygiene)]
#![feature(external_doc)]

extern crate proc_macro;
extern crate proc_macro2;

// #[macro_use]
// extern crate syn;

// #[macro_use]
// use syn::macros

use proc_macro::TokenStream;

use quote::quote;

// use syn;

mod resyn;




#[proc_macro]
pub fn sonic_spin(item: TokenStream) -> TokenStream {
    // let input = syn::parse_macro_input!(item as resyn::item_fn::ItemFn);

    // replicates the input function (after the custom ItemFn parsing)
    // let replica = quote! {
    //    #input
    // };


    let tokens = quote! {
        TODO
    };

    tokens.into()
}





