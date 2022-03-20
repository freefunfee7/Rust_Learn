fn main() {
    // println!("Hello, world!");

    // let s = "Hello";

    // let s = String::from("hello");

    // let mut s = String::from("hello");

    // s.push_str(",world!");
    
    // println!("{}", s);

    // let x = 5;
    // let y = x;

    // let s1 = String::from("yes");
    // let s2 = s1;

    // println!("{}", s1);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1: {}, s2:{}", s1, s2);

    // let x = 5;
    // let y = x;
    
    // println!("x: {}, y:{}", x, y);

    // let s = String::from("hello");

    // take_ownership(s);

    // // println!("{}", s);

    // let x = 5;
    
    // makes_copy(x);

    // println!("{}", x);

    // let s1 = gives_ownership();
    // println!("{}", s1);

    // let s2 = String::from("hello");
    // println!("{}", s2);

    // let s3 = takes_and_gives_back(s2);
    // println!("{}", s3);
    // println!("{}", s2);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of {} is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("s1");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// fn take_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }
