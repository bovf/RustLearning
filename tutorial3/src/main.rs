fn main() {
    let x = 4;
    println!("x is: {}", x); 
    
    {
        let x = x + 2;
        println!("x is: {}", x); 

    }

    let x = "hello";
    println!("x is: {}", x); 
}
