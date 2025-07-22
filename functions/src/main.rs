fn main() {
    println!("Hello, world!");
    another_function();
    parameter_fn(5);
    multi_parameter_fn(5, 'm');

    // Statements and Expressions
    // x = y = 6 is allowed in other languages such as C, but not in rust
    // In rust, assignments (statements) don't return anything
    // An expression block {} can be used:
    let y = {
        let x = 1;
        x + 1 // Note: expressions do not include ending semicolons, otherwise it will not return a value
    };

    println!("{y}");

    let x = returning_function();

    println!("The value of x is: {x}");
}

fn another_function() { // only lowercase and underscores allowed ("snake case")
    println!("Calling another function!");
}

fn parameter_fn(x: i32) { // function parameter types MUST be declared
    println!("Value: {x}");
}

fn multi_parameter_fn(x: i32, units: char) {
    println!("{x}{units}");
}

fn returning_function() -> i32 { // specify return type. Can return early using 'return', but otherwise use last expression (without ;)
    5
    // "5;" will give an error
}