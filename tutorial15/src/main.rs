use std::fmt::Result;

fn main() {
    println!("Hello, world!");

    let number = {
        let x = 3;
        x + 1  // no ; +=> returns a value to let => is an expression
    };
    println!("{}", number);
    let z = add_numbers(1, 3);
    println!("{}", z);
}


fn add_numbers(x: i32, y: i32) -> i32 {
    let result = x + y;
    if result > 10 {
        return result - 10;
    }
    result
}

