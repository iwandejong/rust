fn main() {
    // int live on the stack (they are known in advance)
    let x = 5;
    let y = x; // x will not be invalidated: int has a `Copy` trait

    // String are heap types
    let s1 = String::from("hello");
    let s2 = s1; // s1 is "moved" into s2

    // println!("{s1}, world!"); - s1 will be invalidated.

    // test: changing types.
    let v1: u32 = 5;
    // let v2: i32 = v1.parse().expect("Failed to convert"); doesn't work
    let v2: i32 = v1 as i32; // rather do this, but could cause overflow problems

    // Ownership and functions
    let s = String::from("hello");

    let s = some_function(s); // this call will invalidate `s` in `main()`

    println!("{s}");

    let (s1, len) = calculate_length(s);

    println!("The length of '{s2}' is {len}.");
}

fn some_function(some_string: String) -> String { // add return type here if needed
    println!("{some_string}");

    // or `s` could be returned to `main()`, BUT have to redefine `s`
    some_string
}

fn calculate_length(some_string: String) -> (String, usize) {
    let len = some_string.len();

    (some_string, len)
}