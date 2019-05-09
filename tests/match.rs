#![feature(proc_macro_hygiene)]

mod common;

use sonic_spin::sonic_spin;
use common::Pipe;

#[test]
fn match_a() {
    let alt = match 0 {
        x => x + 2
    };
    let alt = match alt {
        x => x + 10
    };

    let res = sonic_spin!(
        0::(match) {
            x => x + 2
        }::(match)  {
            x => x + 10
        }
    );

    assert_eq!(res, 12);
    assert_eq!(res, alt);
}