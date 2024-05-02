use std::io;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    
    clear_terminal();
    println!("{}", input);
    moving_stars();
    io::stdin().read_line(&mut input).expect("failed to read line");
}

// Function to clean the terminal

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}


// Function to generatie moving stars in the terminal window and animate them
fn moving_stars() {
    let mut x = 0;
    let mut y = 0;
    let mut x_dir = 1;
    let mut y_dir = 1;
    loop {
        print!("{}[{};{}H", 27 as char, y, x);
        print!("*");
        x += x_dir;
        y += y_dir;
        if x == 0 || x == 80 {
            x_dir *= -1;
        }
        if y == 0 || y == 24 {
            y_dir *= -1;
        }
    }
}

