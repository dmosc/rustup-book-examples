fn main() {
    println!("Hello, world!");
    println!("{}", another_function(-5));
}

// Statements end in semicolon whereas expressions not.
// Expressions explicitly return a value from the expression
// evaluation. You can leave a 'tail' expression at the end
// of a function and that's considered the return value
// of the function.
fn another_function(x: i32) -> String {
    if x.is_positive() {
        // You can use `return` statements to exit a function early.
        return "Is positive".to_string();
    }
    "Is negative".to_string()
}
