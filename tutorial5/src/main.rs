fn main() {
    // Compound types
    let tup: (i32, bool, char) = (1, true, 's'); //immutable fixed size type
    let mut tup2: (i32, bool, char) = (1, true, 's'); //mutable fixed size type
    println!("{}", tup.0);
}
