fn main() {
    // println!("Hello, world!");

    // let mut s = String::from("hello world");

    // let s = "Hello world";

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // let s = String::from("hello");

    // let slice = &s[0..2];
    // let slice = &s[..2];

    // let len = s.len();

    // let slice = &s[3..len];
    // let slice = &s[3..];

    // let slice = &s[0..len];
    // let slice = &s[..];

    // let word = first_word(&s);

    // println!("The first word is {}", word);

    // s.clear();

    // println!("The first word is {}", word);


    // let my_string = String::from("Hello World");

    // let word = first_word(&my_string[0..6]);
    // let word = first_word(&my_string[..]);

    // let word = first_word(&my_string);

    // let my_string_literal = "Hello Freefunfee";

    // let word = first_word(&my_string_literal[0..6]);
    // let word = first_word(&my_string_literal[..]);

    // let word = first_word(my_string_literal);


    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    // println!("{}", assert_eq!(slice, &[2, 3]));


}

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }
 
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//            return i; 
//         }
//     }
//     s.len()
// }
