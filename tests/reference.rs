#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]

mod common;

use sonic_spin::sonic_spin;

#[test]
fn reference_normal() {
    sonic_spin! {
        let alt = &4;

        let res = 4::(&);

        assert_eq!(res, &4);
        assert_eq!(alt, res);
    }
}

#[test]
fn reference_multiple() {
    sonic_spin! {
        let alt = &&&4;

        let res = 4::(&)::(&)::(&);

        assert_eq!(res, &&&4);
        assert_eq!(alt, res);
    }
}
