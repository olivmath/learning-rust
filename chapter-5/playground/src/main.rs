mod default;
mod droppable;
mod fibonacci;

use default::Name;
use droppable::Droppable;
use fibonacci::Fibonacci;

fn main() {
    ////////////////////////////////////
    // IMPLEMENTATION OF TRAIT: ITERATOR
    ////////////////////////////////////

    let fib = Fibonacci { this: 0, next: 1 };
    for (i, n) in fib.enumerate().take(10) {
        println!("fib({i}): {n}");
    }

    ////////////////////////////////////
    // IMPLEMENTATION OF TRAIT: DROP
    ////////////////////////////////////

    let a = Droppable { data: "a" };
    {
        let _b = Droppable { data: "b" };
        {
            let _c = Droppable { data: "c" };
            let _d = Droppable { data: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(a);
    println!("Exiting main");

    ////////////////////////////////////
    // IMPLEMENTATION OF TRAIT: DEFAULT
    ////////////////////////////////////

    let name: Name = Default::default();
    println!("{}", name);

    ////////////////////////////////////
    // IMPLEMENTATION OF TRAIT: FN, FNMUT, FNONCE
    ////////////////////////////////////

    let message = "Hello, world!".to_string();
    let mut counter = 0;

    let closure_immutable = || {
        println!("Immutable closure: {}", message);
    };

    let mut closure_mutable = || {
        counter += 1;
        println!("Mutable closure: {}", counter);
    };

    let closure_once = move |msg| {
        println!("Once closure: {}", msg);
    };

    call_immutable_closure(closure_immutable);
    call_mutable_closure(&mut closure_mutable);
    call_once_closure(closure_once, counter);
}

fn call_immutable_closure<F: Fn()>(closure: F) {
    closure();
}
fn call_mutable_closure<F: FnMut()>(closure: &mut F) {
    closure();
}
fn call_once_closure<F: FnOnce(i32)>(closure: F, input: i32) {
    closure(input);
}
