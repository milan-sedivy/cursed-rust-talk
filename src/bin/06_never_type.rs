#![feature(never_type)]

// 06 - Never Type!

use std::process::exit;
use rand::Rng;

const SUCCESS: i32 = 0;
const FAIL: i32 = 1;

fn exit_with_error() -> ! { // we don't need -> !
    eprintln!("This is really annoying");
    exit(FAIL); // because there is an implicit () here
    // it is unreachable, but it is still there. 
}

// We don't really need this, I just wanted to show that you'll find expressions everywhere
macro_rules! unwrap_or_never {
    ($expr:expr) => { // notice the expr ;-)
        match $expr {
            Ok(e) => e,
            Err(_) => exit_with_error(),
        }
    };
}

pub trait UnwrapOrNever<T> {
    fn unwrap_or_never(self) -> T;
}

// You can't have unsecure code if it never runs ;-)
impl<T, E> UnwrapOrNever<T> for Result<T, E> {
    fn unwrap_or_never(self) -> T {
        unwrap_or_never!(self)
    }
}

fn head_or_oops() -> Result<i32, i32> {
    let mut rng = rand::rng();
    let coinflip = rng.random_range(0..2);
    if coinflip > 0 {
        Ok(coinflip)
    } else {
        Err(coinflip)
    }
}


fn jokes_aside() -> i32 {
    // NeverType coerces into any other type so this is valid:
    let x: bool = return 0;
    // Your language server / autocomplete might think it's invalid code though.
    // If you however use "the old rust-analyzer" you might not get it as that
    // uses "cargo check" under the hood.
}

fn jokes_aside2() -> i32 {
    // Still perfectly valid
    if return 1 { 2 } else { 3 }
}

// I think we get the point
fn this_is_a_loop() -> bool {
    let x: bool = loop {
    };
    x
}

// We can either fail to never return successfully
// Or we can fail to never return unsuccessfully
// We can also succeed to never return successfully
// Or we can succeed to never return unsuccessfully
fn main() -> Result<!, !> {
    println!("Never type!");

    println!("{}", jokes_aside());

    println!("{}", jokes_aside2());

    head_or_oops().unwrap_or_never();
    head_or_oops().unwrap_or_never();
    // We successfully never return
    Ok(exit(SUCCESS))
}

// 06 - End of Never Type!
