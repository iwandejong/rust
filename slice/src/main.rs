fn main() {
    // Slice: reference a contiguous sequence of elements rather than the Entire collection
    // Doesn't have ownership

    // Functions do not take ownership of their arguments unless they have to

    // Find the length of the first word of a sentence:
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""
    // `word` is now totally invalid, so we need to find a way to keep the length of `s`
    // we don't want to worry about `s` to know if `word` will work or not (can also not be kept in sync)

    // String slices: reference to a part of a string
    let mut s1 = String::from("hello world");

    println!("{s1}");

    let hello = &s1[0..5];
    let world = &s1[6..11];
    let hello_world = &s1[..];

    // s1.push_str("!"); - Slicing Borrows!

    // let r1 = &s1[..];
    // let r2 = &s1[..];
    // s1 = String::from("not so hello"); // can't because r1 and r2 is already borrowing
    // println!("{r1}");
    // println!("{r2}");

    // drop(s1); // value is being borrowed

    println!("{hello} {world}");
    println!("{hello_world}");

    // now it's easy to understand string literals
    let s_lit = "Hello, world!"; // type: &str, a slice pointing to point of binary, immutable!
    println!("{s_lit}");

    // so instead of `fn first_word(s: &String) -> &str {...}` we could say `fn first_word(s: &str) -> &str {...}`

    // slices can also be used in Arrays!
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    println!("{:?}", slice);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string to bytes

    for (i, &item) in bytes.iter().enumerate() { // iterate over bytes, the pattern used is (i, &item) [& because enumerate() returns a reference]
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
