struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct
struct AlwaysEqual;

fn main() {
    let user1 = build_user(
        String::from("an_email@example.com"),
        String::from("John Doe"),
    );

    println!("Name: {}", user1.username);
    println!("Email: {}", user1.email);

    let user2 = User {
        email: String::from("another_email@example.com"),
        ..user1
    };

    println!("Name: {}", user2.username);
    println!("Email: {}", user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
