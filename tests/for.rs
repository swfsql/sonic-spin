#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]

mod common;

use sonic_spin::sonic_spin;

#[test]
fn for_normal() {
    sonic_spin! {
        let mut _acc = 0;
        for _ in 0..3 {
            _acc += 1;
        };

        let mut acc = 0;
        (0..3)::(for _ in) {
            acc += 1;
        };

        assert_eq!(acc, 3);
        assert_eq!(acc, _acc);
    }
}

#[test]
fn for_nested_pattern() {
    sonic_spin! {
        let mut _acc = 0;
        for x in 0..3 {
            for (a,) in (0..x).map(|b| (b,)) {
                _acc += a;
            }
        };

        let mut acc = 0;
        (0..3)::(for x in) {
            (0..x).map(|b| (b,))::(for (a,) in) {
                acc += a
            }
        };

        assert_eq!(acc, 1);
        assert_eq!(acc, _acc);
    }
}

#[test]
fn for_nested_pattern_labeled() {
    sonic_spin! {
        let mut _acc = 777;
        'outer_: for x in 0..3 {
            'inner_: for (a,) in (0..x).map(|b| (b,)) {
                if a != 0 {
                    break 'outer_;
                }
                _acc += a;
            }
        };

        let mut acc = 777;
        (0..3)::('outer: for x in) {
            (0..x).map(|b| (b,))::('inner: for (a,) in) {
                (a != 0)::(if) {
                    break 'outer;
                };
                acc += a
            }
        };

        assert_eq!(acc, 777);
        assert_eq!(acc, _acc);
    }
}
