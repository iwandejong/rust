fn main() {
    let number = 3;

    if number > 5 {
        println!("Big number!");
    } else if number < 2 {
        println!("Not so big number!");
    } else {
        println!("Just a normal number, {number}.");
    }

    // if number {} will throw an error, because number is not of type 'bool'

    // if is an expression, we can use it on the right side of a 'let' statement
    let x = if number > 5 {5} else {number};
    println!("Number: {}", x);
    println!("Number: {x}");

    // Loops
    // loop {} executes like "while true"
    loop {
        println!("Loop!");
        break;
    }

    let mut counter = 0;
    let result = loop { // result holds the value returned after every loop
        counter += 1;
        if counter == 10 {
            break counter * 2; // this returns the value
        }
    };
    println!("Result is {result}");

    counter = 0;
    while counter < 3 { // while this is true
        println!("{counter}");
        counter += 1;
    }

    // for loop
    let a = [1,2,3,4,5];
    for element in a { // won't go out of bounds! (a typical programming bug)
        print!("{element},");
    }

    for element in (1..5).rev() {
        print!("{element},");
    }
    println!();

    // let's generate n-th fibo!
    // recap: start with 0,1
    // 0 + 1 = 1
    // 1 + 1 = 2
    // 1 + 2 = 3
    // 2 + 3 = 5
    let mut n = 5;
    let mut arr = [0,1];
    while (n-1) > 0 {
        let intermediate = arr[0] + arr[1];
        arr[0] = arr[1];
        arr[1] = intermediate;
        n -= 1;
    }
    println!("fib: {}", arr[1]);
}
