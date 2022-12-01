#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: dbg!(15 * 2),
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels", rect.area());
    println!("Variable `rect` still belongs to main() - {:#?}", rect);
    dbg!(&rect);

    let rect2 = Rectangle {
        width: 15,
        height: 40,
    };
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));

    let square = Rectangle::square(5);
    println!(
        "The area of a square with length 5 is {} square pixels",
        square.area()
    );
}
