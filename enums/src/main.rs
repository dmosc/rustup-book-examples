#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn consume(&self) {
        match self {
            Message::Quit => println!("Quitting"),
            Message::Move { x, y } => println!("Moving to {x} {y}"),
            Message::Write(message) => println!("{message}"),
            Message::ChangeColor(r, g, b) => println!("Changing color to ({r}, {g}, {b})"),
        }
    }
}

fn main() {
    let mut message = Message::Write(String::from("Hello!"));
    message.consume();
    message = Message::Move { x: 5, y: 5 };
    message.consume();
    message = Message::ChangeColor(255, 255, 255);
    message.consume();
    message = Message::Quit;
    message.consume();

    let sure_val: i8 = 1;
    let opt_val: Option<i8> = Some(1);
    println!("{}", sure_val + opt_val.expect("`opt_val` is undefined"));
}
