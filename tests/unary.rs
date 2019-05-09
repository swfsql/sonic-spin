#![feature(proc_macro_hygiene)]

mod common;

use sonic_spin::sonic_spin;
use common::Pipe;

#[test]
fn un_a() {
    let x = &5;
    let alt = *x;

    let res = sonic_spin!(
        x::(*)
    );

    assert_eq!(res, 5);
    assert_eq!(res, alt);
}

