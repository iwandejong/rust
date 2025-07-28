// structs can hold different types
// Definition:
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    // error: &str, - Compiler will complain because of lifetimes, a concept that will be described later on...
} // empty structs are also possible: struct User; - for testing purposes

fn main() {

    // Instance (all fields are required):
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // instance need to be mutable to change values
    // let mut user1 = User {...}
    // user1.email = String::from("anotheremail@example.com");

    let user1 = changeEmail(user1.email);

    // or create user2 with variables from user1 [all variables except for email are MOVED from user1]
    let user2 = User {
        email: String::from("someone@example.com"),
        ..user1 // `email: user1.email` also possible
    };

    // Tuple Structs
    // Don't have names associated, but can have types
    struct Color(i32, i32, i32);
    let red = Color(255, 0, 0);

    // tuple structs require to name the type when destructuring
    let Color(r,g,b) = red;

    let scale = 20;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 10
    };

    // println!("{rect:?}'s area is {area} square units.");
    // println!("{:#?}'s area is {} square units.", rect, area(&rect)); // # prints "structured JSON"

    // !dbg macro: takes ownership, prints file and returns ownership (prints in stderr console instead of stdout)
    dbg!(&rect);

    // but area is limited, rather want area to be a method to Rectangle.
}

fn changeEmail(email: String) -> User {
    User {
        active: true,
        username: String::from("someusername123"),
        email, // no need to say `email: email`
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}