// Since this is a different file than main.rs,
// I need the `pub` keyword to make the function
// public, otherwise it will default to private.

pub fn add(a: i32, b: i32) -> i32 {
    return a + b; // Apparently, neither the semicolon
                  // OR the `return` keyword is mandatory
}

pub fn deduct(a: i32, b: i32) -> i32 {
    a - b
}

// A basic function template:
/*
fn name(parameter: type) -> ReturnType {CODE}
*/