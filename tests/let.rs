#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]

mod common;

use sonic_spin::sonic_spin;

#[test]
fn let_normal() { sonic_spin! {
    let alt = 4;

    4::(let res =);

    assert_eq!(res, 4);
    assert_eq!(alt, res);
}}

#[test]
fn let_pattern() { sonic_spin! {
    let (alt0, alt1) = (3, 4);
    
    (3, 4)::(let (res0, res1) =);

    assert_eq!((res0, res1), (3, 4));
    assert_eq!((alt0, alt1), (res0, res1));
}}
