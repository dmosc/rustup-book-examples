#[derive(Debug)]
enum UsState {
    Alamaba,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alamaba)));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    println!("{:?}", plus_one(None));
    println!("{:?}", plus_one(Some(5)));

    let opt_max: Option<u8> = Some(3u8);
    if let Some(max) = opt_max {
        println!("The max config value is {}", max);
    } else {
        println!("There's no max config value");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<u8>) -> Option<u8> {
    match x {
        None => None,
        Some(val) => Some(val + 1),
    }
}
