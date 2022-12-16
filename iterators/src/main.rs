fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{val}");
    }

    // Methods that consume iterators.
    //
    // v1_iter can't be used since it was moved
    // and used in the previous for-loop.
    let total: i32 = v1.iter().sum();
    println!("{total}");

    // Methods that produce other iterators -- .map(...).
    let v1_x2: Vec<i32> = v1.iter().map(|v| v * 2).collect();
    println!("{:?}", v1_x2);
}
