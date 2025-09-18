use std::cell::RefCell;
use std::rc::Rc;

// 01 - Introduction
fn simple_example() /* -> () */ {
    // Not relevant, just for show
    println!("simple_example");

    // We have an expression 5
    // followed by ; hence there is
    // an `implicit` () at the end
    5;
    // ()
}

fn simple_example_arg(_: ()) /* -> () */ {
    println!("simple_example_arg");
    // ()
}

fn simple_example_i32_arg(arg: i32) /* -> () */ {
    println!("simple_example_i32_arg");
    let arg = arg + 1;
    println!("the arg is: {arg}");
}

fn introduction(useless: ()) {
    simple_example();

    simple_example_arg(useless);

    simple_example_arg({
        println!("Hello from the other side"); simple_example()
    });

    let x = 5;
    simple_example_i32_arg({ println!("the arg is {}", x); x });
}

fn main() {
    println!("Basic expressions!");


}

// 01 - End of Introduction