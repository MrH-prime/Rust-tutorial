use std::io::Write;
use std::mem;

fn main() {
    let x :i32 = 5;
    let y: i32 = 3;
    let z1 = (x as f64) / (y as f64);
    let z2 = x / y;
    println!("int   (x / y)  = {}", z1); // 1.67
    println!("float (x / y)  = {}", z2); // 1

    let size_of_isize = mem::size_of::<isize>();
    println!("size of isize = {}", size_of_isize); // 8

    let size_of_char = mem::size_of::<char>();
    println!("size of char = {}", size_of_char); // 4

    let emoji = '🏆';
    let emoji_unicode = emoji as u32;
    println!("emoji = {}", emoji);
    println!("{emoji} unicode = U+{:04X}", emoji_unicode);
    /*
     : -> start of format specifier
     0 -> output should be zero padded if it's less than specified width
     4 -> Min width of output
     X -> output will be formatted as uppercase hexadecimal
     */

    /*  tuple  */
    let mut tup = (27, 16.5, true);
    let tup1: (i32, f32, bool) = (1, 3.14, true);

    println!("tup1 = {:?}", tup1);
    /*
     ? -> to print value using Debug formating trait
     */
    // destructuring
    let (a1, a2, a3) = tup;
    println!("a1 = {}, b2 = {}, c3 = {}", a1, a2, a3);

    // index
    println!("tup.0 = {}", tup.0);
    tup.0 = 1; // bcz tup is mut
    println!("tup.0 = {}", tup.0);

    /*  array  */
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let arr3 = [3; 5]; // array of 5 elements that all are 3

    println!("arr1 = {:?}", arr1);
    println!("arr2 = {:?}", arr2);
    println!("arr3 = {:?}", arr3);
    print!("arr1[0] = {}\n", arr1[0]);

    for i in arr1 {
        print!("{} ", i);
    }
    print!("\n");

    /* power */
    // pow only accepts u32
    println!("x^y = {}", x.pow(y as u32)); // 125
    // powf accepts f32 or f64 (both base & exponent must be same data type)
    println!("x^-2: {}", (x as f64).powf(-2f64)); // 0.04

    println!("e^x = {}", exponential_power(x as f64));

    /*  loop  */
    let tolerance : f32 = 1e-3;
    let mut count : i32 = 0;
    dbg!(count);
    // std::io::stdout().flush().unwrap();
    'counting_up: loop {
        println!("count = {}", count);
        let mut dec : f32 = 0.1;
        loop {
            println!("\t{:.1}", dec);
            dec += 0.1;
            if (dec - 1.0).abs() < tolerance {
                if count == 9 {
                    break 'counting_up;
                }
                break;
            }

        }
        count += 1;
    }

    /* for */
    // (1..5) : [1, 5)
    for i in 1..5 {
        println!("{}", i);
    }
    // 1..=5 : [1, 5]
    for i in 1..=5 {
        println!("{}", i);
    }
    // to reverse the range
    for i in (1..5).rev() {
        println!("{}", i);
    }
    // waits for a key before closing the program
    print!("Press Enter to Exit!");
    std::io::stdout().flush().unwrap();
    let _ = std::io::stdin().read_line(&mut String::new());
}

fn exponential_power( x : f64) -> f64 {
    // we use exp for e^x (x can be f64 or f32)
    x.exp()
}
