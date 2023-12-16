use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    for s in &args {
        println!("{}", s)
    }
}