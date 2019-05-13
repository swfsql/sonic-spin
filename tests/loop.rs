#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]

mod common;

use sonic_spin::sonic_spin;

#[test]
fn loop_normal() {
    sonic_spin! {
        let mut _acc = 0;
        loop {
            _acc += 1;
            if _acc == 4 {
                break;
            };
        };

        let mut acc = 0;
        {
            acc += 1;
            (acc == 4)::(if) {
                break;
            };
        }::(loop);

        assert_eq!(acc, 4);
        assert_eq!(acc, _acc);
    }
}

#[test]
fn loop_nested_label() {
    let mut _acc = 0;
    'outer_: loop {
        'inner_: loop {
            _acc += 1;
            if _acc == 4 {
                break 'outer_;
            }
        }
    }

    let mut acc = 0;
    sonic_spin!(
        {
            {
                acc += 1;
                (acc == 4)::(if) {
                    break 'outer;
                }
            }::('inner: loop)
        }::('outer: loop)
    );

    assert_eq!(acc, 4);
    assert_eq!(acc, _acc);
}

#[ignore]
#[test]
fn loop_insert_braces() {
    sonic_spin! {
        let do_break = true;
        loop {
            if do_break {
                break
            }
        };

        // TODO: automatically insert the surrouding braces
        { // TODO: remove line
            do_break::(if) {
                break
            }
        } // TODO: remove line
        ::(loop);
    }
}
