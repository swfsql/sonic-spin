#![feature(proc_macro_hygiene)]
use sonic_spin::sonic_spin;

#[test]
fn a() {

    sonic_spin!(
        (5 + 5)
            ::(=)
    );

}
