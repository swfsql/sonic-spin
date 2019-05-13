#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]
mod common;

use sonic_spin::sonic_spin;

#[test]
fn while_normal() {
    sonic_spin! {
        let mut _rep = 3;
        let mut _acc = 0;
        while _rep > 0 {
            _acc += 1;
            _rep -= 1;
        };

        let mut rep = 3;
        let mut acc = 0;
        (rep > 0)::(while) {
            acc += 1;
            rep -= 1;
        };

        assert_eq!(acc, 3);
        assert_eq!(acc, _acc);
    }
}

#[test]
fn while_nested() {
    sonic_spin! {
        let mut _rep = 3;
        let mut _acc = 0;
        while _rep > 0 {
            let mut __rep = 3;
            while __rep > 0 {
                _acc += 1;
                __rep -= 1;
            }
            _rep -= 1;
        };

        let mut rep = 3;
        let mut acc = 0;
        (rep > 0)::(while) {
            let mut rep_ = 3;
            (rep_ > 0)::(while) {
                acc += 1;
                rep_ -= 1;
            };
            rep -= 1;
        };

        assert_eq!(acc, 9);
        assert_eq!(acc, _acc);
    }
}

#[test]
fn while_nested_labeled() {
    sonic_spin! {
        let mut _rep = 3;
        let mut _acc = 0;
        'outer_: while _rep > 0 {
            let mut __rep = 3;
            'inner_: while __rep > 0 {
                _acc += 1;
                __rep -= 1;
                if __rep == 0 {
                    break 'outer_;
                }
            }
            _rep -= 1;
        };

        let mut rep = 3;
        let mut acc = 0;
        (rep > 0)::('outer: while) {
            let mut rep_ = 3;
            (rep_ > 0)::('inner: while) {
                acc += 1;
                rep_ -= 1;
                (rep_ == 0)::(if) {
                    break 'outer;
                }
            };
            rep -= 1;
        };

        assert_eq!(acc, 3);
        assert_eq!(acc, _acc);
    }
}
