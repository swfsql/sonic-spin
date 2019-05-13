#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]
#![feature(box_syntax)]

mod common;

use sonic_spin::sonic_spin;

#[test]
fn box_normal() {
    sonic_spin! {
        let alt = box 2;

        let res = 2::(box);

        assert_eq!(res, box 2);
        assert_eq!(alt, res);
    }
}

#[test]
fn box_let() {
    sonic_spin! {
        let alt = box 2;

        2::(box)::(let res =);

        assert_eq!(res, box 2);
        assert_eq!(alt, res);
    }
}
