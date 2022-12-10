use std::fmt::Display;

fn main() {
    let x = String::from("abcd");
    let y = "xyz";
    println!(
        "The longest string between '{}' and '{}' is: '{}'",
        x,
        y,
        longest(x.as_str(), y)
    );
    println!(
        "The longest string between '{}' and '{}' is (with announcement): '{}'",
        x,
        y,
        longest_with_announcement(x.as_str(), y, "Cool announcement!")
    );
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime Elision rules
// Code patterns related to lifetimes that are written into the Rust compiler to infer
// lifetime annotations.
//
// The compiler follows these 3 rules:

//
// 1. The compiler assigns a lifetime parameter to each parameter that's a reference.
//
// 2. If there's exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
//
// 3. If there are multiple lifetime parameters, one of them being `&self` the lifetime of `&self` is assigned to all
// output lifetime parameters.
//
