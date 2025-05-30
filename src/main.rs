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
        let command_list = ["quit","exit","echo","type"];
        match words[0]{
            "quit" =>break, // quit
            "exit" => return, // exit
            "echo" => { //echo
                if words.len() > 1 {
                    println!("{}",words[1..].join(" "));
                }else{
                    println!(" ");
                }
                
            },
            "type" => { // type
                
                let mut exist = false;
                for i in 0..command_list.len(){
                    if words[1] == command_list[i]{
                        println!("{} is a shell builtin", words[1]);
                        exist = true;
                        break;
                    };
                } ;
                if !exist{
                    println!("{}: not found",words[1]);
                }

            }
            string => println!("{}: command not found", string),
        }
    }
}
