fn main() {
    // let s1 = String::from("hello");

    // let len = calculate_len(&s1);

    // println!("The length of {} is {}", s1, len);

    // let s = String::from("hello");

    // change(&s);

    // let mut s = String::from("hello");

    // change(&mut s);

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // let r1 = &s;
    // let r2 = &s;
    // println!("{}, {}", r1, r2);

    // let r3 = &mut s;

    // println!("{}", r3);

    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// fn  change(some_string: &mut String) {
//     some_string.push_str(",world!");
// }

// fn change(some_string: &String) {
//     some_string.push_str(",world");
// }

// fn calculate_len(s: &String) -> usize {
//     s.len()
// }
