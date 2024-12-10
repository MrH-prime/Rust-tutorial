fn main() {

    /* string slice */
    let s = String::from("hello world");

    // here we just borrow ownership of substrings
    // s1, s2, s3 are not new strings in memory.
    // s1, s2, s3 are string literals (&str)
    let s1 = &s[..5]; // equal to [0..5]
    let s2 = &s[6..]; // equal to [6..len]
    let s3 = &s[..]; // equal to [0..len]

    println!("s1: {}", s1); // hello
    println!("s2: {}", s2); // world
    println!("s3: {}", s3); // hello world

    // here we don't use ref,
    // so we make a new string & s4 will own a new string.
    // here s4 datatype is String
    let s4 = s[..5].to_string();
    println!("s4: {}", s4);



    let mut str = String::from("Hello World");

    let _s5 = &str[0..5];

    //method 1
    let first_word1 = first_word(&str);
    println!("first_word1: {}", first_word1);

    // method 2
    let first_word2 = first_word(&str[..]);
    println!("first_word2: {}", first_word2);

    str.clear(); // this empties str (makes it equal to "") note that str is mut
    // str.clean() is done by mutable borrow

    // now we cant do this
    // bcz s5 is immutable borrow (until below print)
    // and we tried mutable borrow in str.clean()
    // we can't do mutable & immutable borrow at the same time
    // println!("s5: {}", s5); // hello

    /* using slice for array */
    let arr : [i32;5] = [1, 2, 3, 4, 5];

    // here a will be a slice of arr (borrowed)
    // a value type is &[i32]
    let a = &arr[1..3];

    //asser_eq!: a macro for testing equality of 2 expression.
    assert_eq!(a, &[2, 3]);

}

// Defining a function to take a string slice instead of a reference(&String),
// makes our API more general and useful without losing any functionality.
fn first_word(input: &str) -> &str {
    // we need to check string byte by byte for space
    // as_byte() will make string as an array of bytes(returns &[u8])
    let bytes = input.as_bytes();

    // iter():
    // returns an iterator over the items of collection.
    // it borrows the collection & yields references to its elements.

    // enumerate():
    // transforms an iterator to an iterator of pairs.
    // each pairs consists of the index and
    // a reference to the corresponding element.
    // bytes is &[u8] so we have to use & for item for
    // comparing it to b' ' (u8) (now &item is u8 too)

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // from_utf8: Converts a slice of bytes to a string slice
            return std::str::from_utf8(&bytes[..i]).unwrap();
        }
    }
    input
}