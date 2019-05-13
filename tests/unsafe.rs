#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]

mod common;

use sonic_spin::sonic_spin;

#[test]
fn unsafe_normal() {
    sonic_spin! {
        let mut alt = 0;
        unsafe {
            alt += 1;
        };

        let mut res = 0;
        {
            res += 1;
        }::(unsafe);

        assert_eq!(res, 1);
        assert_eq!(res, alt);
    }
}
