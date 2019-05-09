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

#[test]
fn un_b() {
    let x = false;
    let alt = !x;

    let res = sonic_spin!(
        x::(!)
    );

    assert_eq!(res, true);
    assert_eq!(res, alt);
}

#[test]
fn un_c() {
    let x = 2;
    let alt = -x;

    let res = sonic_spin!(
        x::(-)
    );

    assert_eq!(res, -2);
    assert_eq!(res, alt);
}

#[test]
fn un_d() {
    let x = &2;
    let alt = -*x;

    let res = sonic_spin!(
        x::(*)::(-)
    );

    assert_eq!(res, -2);
    assert_eq!(res, alt);
}

