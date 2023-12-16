use std::io::{self, Write};

fn main() {
    
    print!("input: ");
    io::stdout().flush().expect("error"); 
   
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line.");

    println!("Your input: {}", input);
}

