// 06 - Cursed Rust!

// These examples were taken from:
// https://github.com/rust-lang/rust/blob/master/tests/ui/expr/weird-exprs.rs

use std::cell::Cell;
use std::ops::Deref;
// Let's start with the one from the banner

fn function() {
    struct foo;
    impl Deref for foo {
        type Target = fn() -> Self;
        fn deref(&self) -> &Self::Target {
            &((|| foo) as _)
        }
    }
    let foo = foo () ()() ()()() ()()()() ()()()()();
}

// ********************
// Unit type abuse
// ********************
// We've already learned this, it's nothing special, nothing really new
fn funny() {
    fn f(_x: ()) { }
    f(return);
}
fn what() {
    fn the(x: &Cell<bool>) {
        return while !x.get() { x.set(true); };
    }
    let i = &Cell::new(false);
    let dont = {||the(i)};
    dont();
    assert!(i.get());
}

// Let's make it a bit more interesting
fn notsure() {
    let mut _x: isize;
    let mut _y = (_x = 0) == (_x = 0);
    let mut _z = (_x = 0) < (_x = 0);
    let _a = (_x += 0) == (_x = 0);
    let _b = std::mem::swap(&mut _y, &mut _z) == std::mem::swap(&mut _y, &mut _z);
}
fn monkey_barrel() {
    let val: () = ()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=()=();
    assert_eq!(val, ());
}

fn angrydome() {
    loop { if break { } }
    let mut i = 0;
    loop { i += 1; if i == 1 { match (continue) { 1 => { }, _ => panic!("wat") } }
        break; }
}

fn match_nested_if() {
    let val = match () {
        () if if if if true {true} else {false} {true} else {false} {true} else {false} => true,
        _ => false,
    };
    assert!(val);
}

// ********************
// Tokenizer abuse
// ********************
#[rustfmt::skip]
fn fake_macros() -> impl std::fmt::Debug {
    loop {
        if! {
            match! (
                break! {
                    return! {
                    1337
                }
            }
        )
            {}
        }
        {}
    }
}
// ********************
// Path abuse
// ********************
fn inf() {
    pub mod inf {
        pub mod inf {
            pub use super::inf;
            pub struct Inf;
        }
    }
    let _inf: inf::inf::Inf = inf::inf::inf::inf::inf::Inf;
}

fn fishy() {
    assert_eq!(String::from("><>"),
               String::<>::from::<>("><>").chars::<>().rev::<>().collect::<String>());
}
fn fish_fight() {
    trait Rope {
        fn _____________<U>(_: Self, _: U) where Self: Sized {}
    }

    struct T;

    impl Rope for T {}

    fn tug_o_war(_: impl Fn(T, T)) {}

    tug_o_war(<T>::_____________::<T>);
}
// ********************


fn punch_card() -> impl std::fmt::Debug {
    ..=..=.. ..    .. .. .. ..    .. .. .. ..    .. .. .. ..
    ..=.. ..=..    .. .. .. ..    .. .. .. ..    .. ..=.. ..
    ..=.. ..=..    ..=.. ..=..    .. ..=..=..    ..=..=..=..
    ..=..=.. ..    ..=.. ..=..    ..=.. .. ..    .. ..=.. ..
    ..=.. ..=..    ..=.. ..=..    .. ..=.. ..    .. ..=.. ..
    ..=.. ..=..    ..=.. ..=..    .. .. ..=..    .. ..=.. ..
    ..=.. ..=..    .. ..=..=..    ..=..=.. ..    .. ..=..=..
}

fn special_characters() {
    let val = !((|(..):(_,_),(|__@_|__)|__)((&*"\\",'🤔')/**/,{})=={&[..=..][..];})//
        ;
    assert!(!val);
}

fn main() {
    println!("Cursed Rust!");
}

// 06 - End of Cursed Rust!