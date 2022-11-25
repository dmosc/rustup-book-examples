fn main() {
    let condition = false;
    let mut number = if condition { 1 } else { 2 };
    number = 'out_loop: loop {
        number += 1;
        if number == 10 {
            break 'out_loop number * 2;
        }
    };
    println!("{}", number);

    let arr: [i32; 3] = [1, 2, 3];
    for num in arr {
        print!("{num} ");
    }
}
