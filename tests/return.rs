#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]

mod common;

use sonic_spin::sonic_spin;

#[test]
fn return_normal() { sonic_spin! {
    let alt: u32 = || -> u32 {
        loop {
            return 444;
        }
    }();

    let res = || -> u32 {
        loop {
            444::(return);
        }
    }();

    assert_eq!(res, 444);
    assert_eq!(res, alt);
}}