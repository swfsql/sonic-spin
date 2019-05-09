#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]

mod common;

use sonic_spin::sonic_spin;
use common::Pipe;

#[test]
fn if_normal() {
    let alt = if true {
        3
    } else {
        4
    };

    let res = sonic_spin!(
        true::(if) {
            3
        } else {
            4
        }
    );

    assert_eq!(res, 3);
    assert_eq!(res, alt);
}

#[test]
fn if_else() {
    let alt = if false {
        3
    } else {
        4
    };

    let res = sonic_spin!(
        false::(if) {
            3
        } else {
            4
        }
    );

    assert_eq!(res, 4);
    assert_eq!(res, alt);
}

#[test]
fn if_3_branches() {
    let alt = if false {
        3
    } else if true {
        4
    } else {
        5
    };

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
    assert_eq!(res, alt);
}

#[test]
fn if_3_branches_else() {
    let alt = if false {
        3
    } else if false {
        4
    } else {
        5
    };

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
    assert_eq!(res, alt);
}

#[test]
fn if_pipe() {
    let alt = if false {
        0
    } else {
        1
    };
    let alt = alt == 1;
    let alt = if alt {
        2
    } else {
        3
    };

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
    assert_eq!(res, alt);
}

#[test]
fn if_pipe_else() {
    let alt = if false {
        0
    } else {
        1
    };
    let alt = alt == 0;
    let alt = if alt {
        2
    } else {
        3
    };

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
    assert_eq!(res, alt);
}

#[test]
fn if_nested() {
    let mut alt = 0;
    if true {
        if true {
            alt += 1;
        }
    };


    let mut acc = 0;
    sonic_spin!(
        true::(if) {
            true::(if) {
                acc += 1;
            }
        }
    );

    assert_eq!(acc, 1);
    assert_eq!(acc, alt);
}