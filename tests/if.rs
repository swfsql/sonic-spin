#![feature(proc_macro_hygiene)]

mod common;

use sonic_spin::sonic_spin;
use common::Pipe;

#[test]
fn if_a() {
    let res = sonic_spin!(
        true::(if) {
            3
        } else {
            4
        }
    );
    assert_eq!(res, 3);
}

#[test]
fn if_b() {
    let res = sonic_spin!(
        false::(if) {
            3
        } else {
            4
        }
    );
    assert_eq!(res, 4);
}

#[test]
fn if_c() {
    let res = sonic_spin!(
        false::(if) {
            3
        } else if true {
            4
        } else {
            5
        }
    );
    assert_eq!(res, 4);
}

#[test]
fn if_d() {
    let res = sonic_spin!(
        false::(if) {
            3
        } else if false {
            4
        } else {
            5
        }
    );
    assert_eq!(res, 5);
}

#[test]
fn if_e() {
    let res = sonic_spin!(
        false::(if) {
            0
        } else {
            1
        }.pipe(|n| n == 1)::(if) {
            2
        } else {
            3
        }
    );
    assert_eq!(res, 2);
}

#[test]
fn if_f() {
    let res = sonic_spin!(
        false::(if) {
            0
        } else {
            1
        }.pipe(|n| n == 0)::(if) {
            2
        } else {
            3
        }
    );
    assert_eq!(res, 3);
}
