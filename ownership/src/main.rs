fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    // s is 'moved' into s1 and the variable reference is considered
    // to be invalidated.
    // Use `.clone()` to perform a deep copy.
    let s1 = s;
    println!("{s1}");
    take_ownership(s1);
    // Can't use s1 after moving it to take_ownership.

    let mut s = String::from("hello");
    pass_reference(&s);
    // If passed by reference with the ampersand, the variable
    // is borrowed instead of being moved and can be used after
    // the function call.
    println!("Outside function as reference: {s}");

    pass_mutable_reference(&mut s);
    println!("Outside function as mutable reference: {s}");

    let x: u8 = 5;
    make_copy(x);
    // Scalar values implement Copy trait and don't move
    // the reference.
    println!("Outside make_copy: {x}");
}

fn take_ownership(s: String) {
    println!("Inside function: {s}");
}

fn pass_reference(s: &String) {
    println!("Inside function as reference: {s}");
}

fn pass_mutable_reference(s: &mut String) {
    s.push_str(", world!");
    println!("Inside function as mutable reference: {s}");
}

fn make_copy(x: u8) {
    println!("Inside make_copy: {x}");
}
