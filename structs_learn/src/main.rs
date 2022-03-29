// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let user1 = User {
        username: String::from("freefunfee7"),
        email: String::from("freefunfee7@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    // user1.username = String::from("Billy");

    println!("{}", user1.username);

    // let user2 = User {
    //     username: user1.username,
    //     email: String::from("free@gmail.com"),
    //     active: user1.active,
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User {
        email: String::from("free7@gmail.com"),
        // username: String::from("test"),
        ..user1
    };

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }

}
