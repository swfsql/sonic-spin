#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]
#![feature(label_break_value)]

mod common;

use sonic_spin::sonic_spin;

#[test]
fn block_label() {
    sonic_spin! {
        let mut alt = 0;
        'alt_label: {
            alt += 1;
        };

        let mut res = 0;
        {
            res += 1;
        }::('res_label:);

        assert_eq!(res, 1);
        assert_eq!(res, alt);
    }
}
