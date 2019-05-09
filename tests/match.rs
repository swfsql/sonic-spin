#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]

mod common;
use sonic_spin::sonic_spin;

#[test]
fn match_cascade() { sonic_spin! {
    let alt = match 0 {
        x => x + 2
    };
    let alt = match alt {
        x => x + 10
    };

    let res = 0::(match) {
        x => x + 2
    }::(match)  {
        x => x + 10
    };

    assert_eq!(res, 12);
    assert_eq!(res, alt);
}}

#[test]
fn match_nested() { sonic_spin! {
    let alt = match 0 {
        a @ 0..=3 => match a {
            y => y + 1000,
        },
        _x => 5
    };

    let res = 0::(match) {
        a @ 0..=3 => a::(match) {
            y => y + 1000,
        },
        _x => 5,
    };

    assert_eq!(res, 1000);
    assert_eq!(res, alt);
}}