fn main() {
    let s = String::from("Hello world");
    let word = first_word(&s[..]);
    println!("{}", word);
    println!("{}", first_word("String literal"));

    let arr: [i8; 5] = [1, 2, 3, 4, 5];
    let arr_slice = &arr[0..2];
    assert_eq!(arr_slice, &[1, 2]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
