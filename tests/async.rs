#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]
#![feature(async_await)]
#![feature(impl_trait_in_bindings)]
#![feature(futures_api)]

mod common;

use sonic_spin::sonic_spin;

#[test]
fn async_normal() {
    sonic_spin! {
        let alt: impl std::future::Future = async { (); };
        let res: impl std::future::Future = { (); }::(async);
    }
}
