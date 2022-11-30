struct User {
    active: bool,
    username: String,
    email: String,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user"),
        email: String::from("email@example.com"),
    };
    println!("{}, {}, {}", user1.email, user1.username, user1.active);
    user1.email = String::from("new_email@example.com");
    println!("{}", user1.email);

    let user2 = User { ..user1 };
    println!("{}", user2.email);

    struct RGB(i32, i32, i32);
    let rgb_instance = RGB(45, 45, 45);
    println!("({} {} {})", rgb_instance.0, rgb_instance.1, rgb_instance.2);
}
