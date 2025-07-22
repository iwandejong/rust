fn main() {
    // scoping
    let x = 5;

    {
        let x = x + 2;
        println!("Value {x}");
    }
    println!("Value {x}");

    // type inference
    let spaces = "   ";
    let spaces = spaces.len();

    println!("{}", spaces);

    // data types
    let guess: u32 = "42".parse().expect("Not a number!");
    // let guess = "42".parse().expect("Not a number!"); - will cause a problem: parse() needs to know a type

    println!("{}", guess);

    // scalar types
    // single value: integers, floating-point, numbers, Boolean, and characters
    // signed: +/-; unsigned: +

    let x = 2.0; // f64 (default)
    let y: f32 = 3.0; // f32

    // numeric operations
    // mod: %

    // char literals are expressed in single quote literals ('')

    // compound types
    // tuple
    let tup: (i32, f64, u8) = (500_00, 6.4, 1);
    // it is also possible to destructure a tuple value:
    let (x,y,z) = tup;
    println!("{y}");
    // ...or it can be accessed directly:
    let five_hundred = tup.0;
    println!("{five_hundred}");

    // arrays in rust have a fixed length (useful when data )
    // arrays stores data on the stack
    let a = [1,2,3,4,5];
    // explicit type and length:
    let a: [i32; 5] = [1,2,3,4,5];
    // same element n times
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    // access array elements
    println!("{}", a[0]);
    // rust protects against invalid memory outputs when accessing incorrect indices
}
