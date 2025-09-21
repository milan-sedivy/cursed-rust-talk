use std::cell::RefCell;
use std::rc::Rc;

fn simple_example() /* -> () */
{
    // Not relevant, just for show
    println!("simple_example");

    // We have an expression 5
    // followed by ; hence there is
    // an `implicit` () at the end
    5;
    // ()
}

fn simple_example_arg(_: ()) /* -> () */
{
    println!("simple_example_arg");
    // ()
}

fn simple_example_i32_arg(arg: i32) /* -> () */
{
    println!("simple_example_i32_arg");
    let arg = arg + 1;
    println!("the arg is: {arg}");
}

// 02 - A little bit more interesting

fn a_little_bit_more_interesting() {
    let mut first_time = 0i32;

    for _ in 0..2 {
        simple_example_i32_arg(if first_time == 0 {
            first_time += 1;
            0
        } else {
            first_time
        });
    }
}

// 02b - more useful real world example
struct Foo {
    x: String,
}

struct Bar {
    y: Rc<RefCell<Foo>>,
}

struct FooBuilder {
    x: Option<String>,
}

impl FooBuilder {
    fn new() -> Self {
        FooBuilder { x: None }
    }
    fn change_x(mut self, x: String) -> Self {
        self.x = Some(x);
        self
    }

    fn build(self) -> Foo {
        match self.x {
            None => Foo {
                x: "Bar".to_string(),
            },
            Some(x) => Foo { x },
        }
    }
}

fn obtain_string() -> String {
    "Fighters".to_string()
}
fn more_useful_example() {
    let use_foo_fighters = true;
    let foo = {
        let foo_builder = FooBuilder::new();
        let foo_fighter = if use_foo_fighters {
            obtain_string()
        } else {
            return;
        };
        foo_builder.change_x(foo_fighter).build()
    };
    let bar = Bar {
        y: Rc::new(RefCell::new(foo)),
    };
    let what_did_i_change = {
        let baz = "Baz".to_string();
        bar.y.borrow_mut().x = baz.clone();
        baz
    };
    println!("what did I change?: {}", what_did_i_change);
    println!("I can borrow safely?: {}", bar.y.borrow().x);
}

struct ThisIs(usize);

fn work_damnit(_: usize, foo: &RefCell<ThisIs>) {
    foo.borrow_mut();
}

//Danger danger
fn will_it_panic() {
    let foo = RefCell::new(ThisIs(0xBAAAAAAD));
    // Will it Panic??
    work_damnit(foo.borrow().0, &foo);

    // **whispers** ~~expressions~~ **whispers**
    work_damnit({ foo.borrow().0 }, &foo);
}

fn main() {
    println!("A little bit more interesting!");
    will_it_panic()
}
// 02 - End of A little bit more interesting
