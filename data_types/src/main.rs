use std::io;

fn main() {
    // Scalar Types
    let _guess: u32 = "32".parse().expect("Not a number!");

    let _x = 2.0;

    let _y: f32 = 32.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 93.0 - 52.3;

    // multiplication
    let product = 4 * 30;
    
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;

    // remainder
    let remainder = 43 % 5;

    println!("sum is: {}", sum);
    println!("difference is: {}", difference);
    println!("product is: {}", product);
    println!("quotient is: {}", quotient);
    println!("floored is: {}", floored);
    println!("remainder is: {}", remainder);


    let t = true;
    let f: bool =  false;

    println!("t is: {}", t);
    println!("f is: {}", f);

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😁';

    println!("c is: {}", c);
    println!("z is: {}", z);
    println!("heart_eyed_cat is: {}", heart_eyed_cat);

    // Compound Types
    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, _y, _z) = tup;

    println!("The value of y is: {}", _y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("five_hundred is: {}", five_hundred);
    println!("six_point_four is: {}", six_point_four);
    println!("one is: {}", one);

    // The Array Type
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January", "February", "March", "April",
        "May", "June", "July", "August",
        "September", "October", "November", "December"
    ];

    // println!("months is: {}", months);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let x1 = [3; 5];
    let x2 = [3, 3, 3, 3, 3];
    if x1 == x2 {
        println!("Yes");
    }

    let first = a[0];
    let second = a[1];

    println!("first is: {}", first);
    println!("second is: {}", second);

    // invalid array element access

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
                       .trim()
                       .parse()
                       .expect("Index entered was not a number");
    
    let element = a[index];

    println!("The value of the element at index {} is: {}",
            index, element);
}
