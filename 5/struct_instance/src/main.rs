#[derive(Debug)] // to println! with struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{:?}", user2); // User { username: "anotherusername567", email: "another@example.com", sign_in_count: 1, active: true }

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black.1); // 0
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}