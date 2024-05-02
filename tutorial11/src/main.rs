fn main() {
    let cond = (2 as f32) < 2.3;
    // and or not
    // && || !
    let cond2 = !(false && cond);

    // ! >> && >> || --- this is the precedent order

    println!("{}", cond2);

}
