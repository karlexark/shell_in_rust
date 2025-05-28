#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    let mut input = String::new();
    loop{   
        print!("$ ");
        io::stdout().flush().unwrap();
    
    // Wait for user input
        
        io::stdin().read_line(&mut input).unwrap();
        match input.trim(){
            "exit 0" => break,
            string => println!("{}: command not found", string),
        }
    }
}
