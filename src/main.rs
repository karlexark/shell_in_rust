#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    
    loop{   
        print!("$ ");
        io::stdout().flush().unwrap();
    
    // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty(){
            continue;
        }
        let words: Vec<&str> = input.split_whitespace().collect();
        match words[0]{
            "quit" => break,
            "exit 0" => return,
            "echo" => println!("{}",words[1..].join(" ")),
            string => println!("{}: command not found", string),
        }
    }
}
