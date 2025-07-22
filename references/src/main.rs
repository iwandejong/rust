fn main() {
    // if we want to use the `String` after the call, we can just pass reference
    let mut s1 = String::from("Hello");

    let len = some_function(&s1); // allows to refer to some value without taking ownership of it, won't be dropped
    some_mutable_function(&mut s1);

    println!("{s1} has a length of {len}");

    // &: referencing
    // *: dereferencing

    // Mutable references have one big restriction: can only have 1 mut ref to a value
    // let r1 = mut& s1;
    // let r2 = mut& s1;
    // -> invalid, but prevents data races (2+ pointers accessing data at the same time, no sync method, at least one pointer is used to write data)
    // can use scopes {...}

    // let r1 = &s1;
    // let r2 = &s1;
    // let r3 = &mut s1; // problem! (...unless r1 and r2 is "deallocated earlier on")

    // Dangling Pointers
    // let reference_to_nothing = dangle();

    // Only 1 mutable reference or n immutable reference(s)
    // References must always be valid (can't be dangling)
}

fn some_function(some_var: &String) -> usize { // can't modify some_var (references are immutable)
    some_var.len()
} // some_var goes out of scope here

// to make the reference variable mutable, we add the `mut` keyword and also change s1 to `let mut`
fn some_mutable_function(some_var: &mut String) {
    some_var.push_str(", world!");
}

// fn dangle() -> &String {
//    let s = String::from("hello"); // s goes out of scope, but the reference is returned...

//    &s
//    s // rather just return this
// }