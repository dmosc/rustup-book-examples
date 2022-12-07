use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let size: u8 = rng.gen_range(1..10);
    let vals: Vec<u16> = (0..size).map(|_| rng.gen_range(0..10)).collect();
    println!("{:?}", vals);

    let vals1 = vec![1.2, 1.3];
    println!("{:?}", vals1);

    let second = &vals1[1];
    println!("The second value is {second}");

    let second = vals1.get(1);
    match second {
        Some(second) => println!("The second value using `get` is still {second}"),
        None => println!("There's no second value!"),
    }

    for i in &vals1 {
        println!("{i}");
    }
    println!("`vals1` as immutable reference: {:?}", vals1);

    let mut vals1 = vals1;
    for i in &mut vals1 {
        *i += 1_f64;
    }
    println!("`vals1` as mutable reference: {:?}", vals1);

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(5),
        SpreadSheetCell::Float(1_f64),
        SpreadSheetCell::Text(String::from("Hello world!")),
    ];
    for cell in row {
        match cell {
            SpreadSheetCell::Int(val) => println!("An integer {val}"),
            SpreadSheetCell::Float(val) => println!("A float {val}"),
            SpreadSheetCell::Text(val) => println!("A text {val}"),
        }
    }
}
