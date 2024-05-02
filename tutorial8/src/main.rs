use std::result;

fn main() {
    let x: u8 = 12; // 0 - 255
    let y: i8 = 10; // -128 - 127
                    //
    let z = u128::MAX;
    println!("{}", z);


    let h: f32 = 257.0;
    let j: f32 = 10.0;
    let result = h / j;

    println!("{}", result)
}
