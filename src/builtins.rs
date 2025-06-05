//All the imports
use crate::autocompletion::HelpTab;
use rustyline::history::{DefaultHistory, History};
use rustyline::Editor;
use std::path::Path;
use std::process::Command;

// its the echo command function which it call when the user write echo blabla in the terminal
pub fn cmd_echo(args: &[&str]) {
    println!("{}", args.join(" ")); // we print the args wich is all the line but "echo "
}

// its the type command function which is call when the user write type blabla on the terminal
pub fn cmd_type(args: &[&str], paths: &Vec<String>) {
    //INPUTS : args an array of &str : it conatins the argument; path a Vec of string, it contains all the dir of the environement
    let args_len = args.len(); // recup of the len of the args

    match args_len {
        0 => return, // if there is nothing (the user only wrote type) we return and on the terminal will print the next line
        1 => {
            // if there is one arg
            match args[0] {
                "exit" | "echo" | "type" | "history" | "pwd" | "cd" | "ls" => {
                    println!("{} is a shell builtin", args[0])
                } // fisrt we look into the builtins list

                _ => {
                    // if its not we gonna look into the dir to find if the command exist
                    let mut found = false;
                    for dir in paths.iter() {
                        // for every dir in the paths
                        let full_path = Path::new(dir).join(args[0]); // we create a full path with the idir and the command we are looking for
                        if full_path.is_file() {
                            // if this is a file
                            found = true; // we found it
                            println!("{} is {}", args[0], full_path.display()); // we print the command and the directory where it is
                            break;
                        }
                    }
                    if !found {
                        // if after lokking into all the dir we didn't find anything
                        println!("{}: not found", args[0]); // we print it to the user
                    }
                }
            }
        }
        _ => {
            // if there are more than one arg
            println!("type : too many arguments");
            return;
        }
    }
}

pub fn cmd_history(mut n: usize, edit: &Editor<HelpTab, DefaultHistory>) {
    //history command function
    //INPUTS : n an usize : it is the number of line that the user wants to see; edit an editor : it got in memory the hiostory of the line
    if n > edit.history().len() {
        // if the user want to print more line that we have in memory
        n = edit.history().len(); // we change the value of n with the maximum number of line that we have in memory
    }
    for (i, entry) in edit.history().iter().enumerate() {
        //the list of line is read and enumarate
        if i >= edit.history().len() - n || n == 0 {
            // the first condtition permit to print the number of line ask by the user exemple : we have 100 line in memory, the user wants to see 10 line so n = 10 and only the line 90 to 100 have to be print
            // if n = 0 that means the user wants to see all the lines so we print no matter the value of i
            println!("{}  {}", i, entry);
        }
    }
}

pub fn cmd_pwd() {
    // pwd command function
    match std::env::current_dir() {
        // trying to recup the current dir
        Ok(path) => println!("{}", path.display()), // if it work, print the current dir
        Err(e) => eprintln!("Erreur lors de l'exécution de pwd : {}", e), //if its not print an error
    }
}

pub fn cmd_cd(mut path: String) {
    if path == "~" {
        if let Some(pathbuf) = dirs::home_dir() {
            path = pathbuf.to_string_lossy().to_string();
        }
    }
    if let Err(_e) = std::env::set_current_dir(&path) {
        eprintln!("cd: {}: No such file or directory", path);
    }
}

pub fn cmd_ls(path: &str) -> Vec<String> {
    match std::fs::read_dir(path) {
        Ok(file_list) => {
            let mut file_list_str = Vec::new();
            for os_file in file_list {
                match os_file {
                    Ok(file) => {
                        let os_name = file.file_name();
                        file_list_str.push(os_name.to_string_lossy().to_string());
                    }
                    Err(_) => continue,
                }
            }
            file_list_str.sort(); //TODO rentre le tri plus robuste
            return file_list_str;
        }
        Err(e) => {
            eprintln!(
                "ls : impossible de lire de les fichiers de {} : {}",
                path, e
            );
            return Vec::new();
        }
    }
}

pub fn cmd_ext(args: &[&str], paths: &Vec<String>) {
    let args_len = args.len();
    match args_len {
        0 => return,
        _ => {
            let mut found = false;
            for dir in paths.iter() {
                let full_path = Path::new(dir).join(args[0]);
                if full_path.is_file() {
                    found = true;
                    run_external(&full_path.to_string_lossy(), &args[1..]);
                    break;
                    //}
                }
            }
            if !found {
                println!("{}: program not found", args[0]);
            }
        }
    }
}

pub fn run_external(program_name: &str, args: &[&str]) {
    match Command::new(program_name).args(args).status() {
        Ok(_) => (),
        Err(e) => eprintln!("Erreur lors de l'execution de {} : {}", program_name, e),
    };
}
