# Sonic Spin

Declares a macro that turns the `::()` postfix operator into a general postfix operator.  
Mostly by re-defining structures from the `syn` for parsing.  

## Example

```rust
#![feature(proc_macro_hygiene)]
use sonic_spin::sonic_spin;

sonic_spin! {
    let res = 0::(match) {
        x => x + 2
    }::(match)  {
        x => x + 10
    };
    assert_eq!(res, 12);


    let mut acc = 0;
    (0..3)::(for _ in) {
        acc += 1;
    };
    assert_eq!(acc, 3);
}
```

See `tests/` for further examples.  

## Motivation

Given some of the many discussions regarding the `await` syntax and the resulting possibility of general postfix operators, this crate explores such possibility with the "sonic-spin" operator (`::()`).  
If the await syntax turned out to be "prefix await" `await expr`, then this crate would enable the syntax `expr::(await)` similarly to the `?` postfix operator.

Some of the mentioned discussions:
- https://internals.rust-lang.org/t/idea-universal-pipelining-a-k-a-making-await-generic/9973
- https://github.com/rust-lang/rfcs/pull/2442


## Further work

- Explore if/how postfix macros could work.
- Test with the await case (even if it's already a postfix operator).
- Explore auto bracing addition.

## Pipe operations

For piped operations, see the `tests/common.rs::Pipe` trait and it's usage in `tests/if.rs::if_pipe` function. ie. sonic-spin is not required for such operations.

## Notes

This is a draft and is based on this suggestion:  
https://github.com/rust-lang/rfcs/issues/2698