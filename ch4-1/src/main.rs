fn main() {

    /* integers use Copy  */
    let x = 5;
    let y = x;
    println!("address of x: {:p}", &x); //  0x42574ff838
    println!("address of y: {:p}", &y); //  0x42574ff83c
    // deep copy and shallow copy have same result for int

    /* string literal (in text segment) */
    // str data type is &str
    let str: &str = "Hello";

    /* String object (in heap) */
    // s data type is String.
    // String::from Converts a &str into a String.
    // s1 is stored in stack but String obj is in Heap
    let mut s1 = String::from(str);

    // push_str() appends literals to String.
    s1.push_str(", World!");

    // Both s1 & s2 point to the same address in Heap
    let mut s2 : String = s1;
    // here move occurs (not copy)
    // s1 moves into s2
    // due to avoidance of "double free error",
    // now s1 is out of scope & we can't use it anymore.
    println!("s2 = {s2}");

    s2 = String::from("Ahoy!");
    // the prev value of s2 goes out of scope, so
    // Rust will run the drop function on it
    // & its memory in heap will be freed.
    println!("s2: {}", s2);

    // we make deep copy by clone()
    // deep copy can be expensive.
    let s3 : String = s2.clone();
    // now s3 and s2 are both available in stack, and
    // they point to two different address of Heap,
    // we have 2 Ahoy object in heap, one for each.

    println!("s2 = {s2}\ns3 = {s3}");

    /*  ownership & function  */
    let s4 : String = String::from("Hello");

    takes_ownership(s4);
    // here s4 moved into function,
    // so we s4 is out of scope & we can't use it anymore.

    makes_copy(x);
    // x copies to the function
    // so we can use x still

    /* functions can return multiple values using tuple */
    let s5 : String = String::from("Hello");

    let (str, length) = calculate_length(s5);
    // here we can't use s5 anymore, so we need string in return value
    println!("the length of {str} is {length}");

    let mut s6 = String::from("Hello");
    let length = calculate_length2(&s6);
    // here we pass by reference, so the ownership of s6 didn't change.
    // so we can use s6 & we don't need function return string
    println!("the length of {s6} is {length}");

    // references are immutable by default
    // we can have multiple immutable ref at the same time.

    // we can have mut ref ( Note that one variable only can have one mut ref at the same time)
    // we can't have immutable ref & mutable ref at the same time too!

    // here we change s6 by pass by ref
    string_push(&mut s6);
    println!("s6 = {s6}");

    /*
     here we will get ERROR
     bcz before release of s8, we tried another &mut s7
    */
    // let mut s7 = String::from("Hello");
    // let s8 = &mut s7;
    // string_push(s8);
    // string_push(&mut s7);
    // println!("s8 = {s8}"); // here s8 is released!

    // this is true version of above
    let mut s7 = String::from("Hello");
    let s8 = &mut s7;
    string_push(s8);
    println!("s8 = {s8}"); // here s8 is released!
    string_push(&mut s7); // we can again use &mut s7
    println!("s7 = {s7}"); // s7 = Hello, World!, World!

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}

fn calculate_length2(some_string: &String) -> usize {
    some_string.len()
}

fn string_push(some_string: &mut String) {
    some_string.push_str(", World!");
}
