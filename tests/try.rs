#![feature(proc_macro_hygiene)]
#![allow(unused_parens)]

#![feature(try_blocks)]

mod common;
use sonic_spin::sonic_spin;

#[test]
fn try_normal()  { sonic_spin!{
    let alt: Result<u32, ()> = try { 8 };
    let res: Result<u32, ()> = { 8 }::(try);

    assert_eq!(res, Ok(8));
    assert_eq!(res, alt);
}}