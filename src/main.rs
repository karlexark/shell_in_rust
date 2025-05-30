#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;

use std::os::unix::fs::PermissionsExt;

fn main() {
    let path_value = std::env::var("PATH").unwrap();
    let paths: Vec<&str> = path_value.split(':').collect();
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
            ["exit","0"] => return, 
            ["echo", args @ ..] => cmd_echo(args),
            ["type", args @ ..] => cmd_type(args,&paths),
            _ => println!("{}: command not found",input),
        }
    }
}

fn cmd_echo(args: &[&str]){
    println!("{}",args.join(" "));
}

fn cmd_type(args: &[&str],paths: &Vec<&str>){
    let args_len = args.len();

    match args_len{
        0 => return,
        1 =>{
            match args[0]{
                "exit" | "echo" | "type" => println!("{}: is a shell builtin", args[0]),

                _ => {
                        let mut found = false;
                        for dir in paths.iter() {
                            let full_path = format!("{}/{}", dir, args[0]);
                            if Path::new(&full_path).is_file() {
                                //let meta = std::fs::metadata(&full_path);
                                //let mode = meta.permissions().mode();
                                //if mode & 0o111 !=0{
                                found = true;
                                println!("{} is {}",args[0],full_path);
                                break;
                                //}
                            }
                        }
                        if !found {
                            println!("{}: not found", args[0]);
                        }
                }
            }
        },
        _ => {
            println!("type : too many arguments");
            return;
        },

    }
}

