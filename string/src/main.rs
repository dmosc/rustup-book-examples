fn main() {
    let data = "こんにちは";
    let mut s = data.to_string();
    let exclamation_mark = "!";
    s += exclamation_mark;
    s.push_str(exclamation_mark);
    println!("{}{}", s, exclamation_mark);
    s = format!("!!!{s}!");
    println!("{}", s);

    let cyrillic_hello = String::from("Здравствуйте");
    let bytes = cyrillic_hello.as_bytes();
    // Total bytes are 24...
    println!("Total bytes {}\n{:?}", bytes.len(), bytes);

    // Slice the string to extract all two bytes from the first letter З.
    println!("{}", &cyrillic_hello[0..2]);

    for c in cyrillic_hello.chars() {
        println!("{c}");
    }
    println!("{cyrillic_hello}");
}
