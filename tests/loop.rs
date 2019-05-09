#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]

mod common;

use sonic_spin::sonic_spin;
use common::Pipe;

// #[test]
// fn loop_normal() { sonic_spin! {
//     let mut _acc = 0;
//     loop {
//         _acc += 1;
//         if _acc == 4 {
//             break;
//         }
//     }

//     let mut acc = 0;
//     {
//         acc += 1;
//         (acc == 4)::(if) {
//             break;
//         }
//     }::(loop);

//     assert_eq!(acc, 4);
//     assert_eq!(acc, _acc);
// }}

/*
#[test]
fn loop_nested_label() {
    let mut _acc = 0;
    'outer: loop {
        'inner: loop {
            _acc += 1;
            if _acc == 4 {
                break 'outer;
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
*/