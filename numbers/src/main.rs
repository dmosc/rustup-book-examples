use std::io;

fn main() {
    let number: u8 = 0b11111111;
    let mut diff = String::new();
    io::stdin()
        .read_line(&mut diff)
        .expect("Failed to read line.");
    let diff: u8 = match diff.trim().parse() {
        Ok(diff) => diff,
        Err(err) => panic!("{err}"),
    };
    let sum = number.overflowing_add(diff);
    println!(
        "[.overflowing_add] -- Value: {}, overflowed: {}",
        sum.0, sum.1
    );
    println!("[.saturating_add] -- {}", number.saturating_add(diff));
}
