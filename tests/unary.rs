#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]

mod common;
use sonic_spin::sonic_spin;

#[test]
fn un_deref() {
    let x = &5;
    let alt = *x;

    let res = sonic_spin!(
        x::(*)
    );

    assert_eq!(res, 5);
    assert_eq!(res, alt);
}

#[test]
fn un_not() {
    let x = false;
    let alt = !x;

    let res = sonic_spin!(
        x::(!)
    );

    assert_eq!(res, true);
    assert_eq!(res, alt);
}

#[test]
fn un_minus() {
    let x = 2;
    let alt = -x;

    let res = sonic_spin!(
        x::(-)
    );

    assert_eq!(res, -2);
    assert_eq!(res, alt);
}

#[test]
fn un_minus_deref() {
    let x = &2;
    let alt = -*x;

    let res = sonic_spin!(
        x::(*)::(-)
    );

    assert_eq!(res, -2);
    assert_eq!(res, alt);
}