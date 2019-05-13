#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]

mod common;

use sonic_spin::sonic_spin;

#[test]
fn break_normal() {
    sonic_spin! {
        let alt = loop {
            break 777;
        };

        let res = loop {
            777::(break);
        };

        assert_eq!(res, 777);
        assert_eq!(res, alt);
    }
}

#[test]
fn break_labeled() {
    sonic_spin! {
        let alt = 'outer_: loop {
            'inner_: loop {
                break 'outer_ 555;
            }
        };

        let res = 'outer: loop {
            'inner: loop {
                555::(break 'outer);
            }
        };

        assert_eq!(res, 555);
        assert_eq!(res, alt);
    }
}
