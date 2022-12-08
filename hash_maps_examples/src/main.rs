use std::collections::HashMap;

fn main() {
    let nums = vec![1, 9, 2, 8, 3, 7, 4, 6, 5, 0, 4, 4, 4];
    println!("The median of {:?} is {}", nums, median(nums.to_vec()));

    let num_mode = mode(nums.to_vec()).expect("There's no mode");
    println!(
        "The mode is {} and it repeats {} times",
        num_mode.0, num_mode.1
    );
}

fn median(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    *nums.get(nums.len() / 2).unwrap_or(&0)
}

fn mode(nums: Vec<i32>) -> Option<(i32, u32)> {
    let mut freqs: HashMap<i32, u32> = HashMap::new();
    for &num in &nums {
        let entry = freqs.entry(num).or_insert(0);
        *entry += 1;
    }
    freqs.into_iter().max_by_key(|v| v.1)
}
