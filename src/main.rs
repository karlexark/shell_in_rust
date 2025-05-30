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
        match words.as_slice(){
            [""] => continue,
            ["exit",0] => return, // exit
            ["echo", args @ ..] => cmd_echo(args),
            ["type", args @ ..] => cmd_type(args),
        }
    }
}

fn cmd_echo(args: &[&str]){
    println!("{}",args.join(" "));
}

fn cmd_type(args: &[&str]){
    let args_len = args.len();

    match args_len{
        0 => return,
        >1 => {
            println!("type : too many arguments");
            return;
        },
        _ =>{
            match args[0]{
                "exit" | "echo" | "type" => println!("{}: is a shell builtin", args[0]),
                _ => println!("{}: not found",args[0]),
            }
        }

    }
}