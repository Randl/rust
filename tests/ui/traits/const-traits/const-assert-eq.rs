pub const fn foo(a: i8, b: i8) -> i8 {
    assert!(a == b);
    a
}

pub const fn bar(a: i8, b: i8) -> i8 {
    assert_eq!(a, b);
    a
}


const C: i8 = foo(1,1);
const D: i8 = bar(1,1);
const G: i8 = foo(1,2);
//~^ ERROR: evaluation panicked: assertion failed: a == b
const E: i8 = bar(1,2);
//~^ ERROR: evaluation panicked: Assertion failed

fn main() {}
