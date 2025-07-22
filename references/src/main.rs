fn main() {
    // if we want to use the `String` after the call, we can just pass reference
    let s1 = String::from("Hello");

    let len = some_function(&s1); // allows to refer to some value without taking ownership of it, won't be dropped

    println!("{s1} has a length of {len}");

    // &: referencing
    // *: dereferencing
}

fn some_function(some_var: &String) -> usize {
    some_var.len()
} // some_var goes out of scope here
